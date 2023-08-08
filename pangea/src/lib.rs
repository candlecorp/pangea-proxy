mod wick {
    wick_component::wick_import!();
}
use wick::*;

#[async_trait::async_trait(?Send)]
impl GeolocateOperation for Component {
    type Error = anyhow::Error;
    type Outputs = geolocate::Outputs;
    type Config = geolocate::Config;

    async fn geolocate(
        mut ip: WickStream<String>,
        mut outputs: Self::Outputs,
        ctx: Context<Self::Config>,
    ) -> Result<(), Self::Error> {
        while let Some(ip) = ip.next().await {
            let ip = propagate_if_error!(ip, outputs, continue);

            let (mut get_geolocate_response, mut get_geolocate_response_body) =
                ctx.provided().pangea_api.ip_geolocate(once(ip))?;

            while let (Some(get_geolocate_response), Some(get_geolocate_response_body)) = (
                get_geolocate_response.next().await,
                get_geolocate_response_body.next().await,
            ) {
                //Throw error if the response is not valid
                let response = propagate_if_error!(get_geolocate_response, outputs, continue);
                let body = propagate_if_error!(get_geolocate_response_body, outputs, continue);

                if response.status != types::http::StatusCode::Ok {
                    outputs.status.error(format!(
                        "Pangea api returned status code {}",
                        response.status
                    ));
                    continue;
                }

                // Attempt to retrieve the 'status' field from the API response body.
                let status = body
                    .get("status")
                    .and_then(|s| s.as_str().map(|s| s.to_string()));

                // Handle based on the status received.
                match &status {
                    Some(status) => println!("status: {}", status),
                    None => {
                        outputs.status.error("IP Not found");
                        continue;
                    }
                };

                let status = status.unwrap();

                if status != "Success" {
                    //send stream of errors to wick
                    outputs
                        .status
                        .error(format!("Pangea api returned status {}", status));
                    continue;
                } else {
                    println!("status: {}", status);

                    // Attempt to extract geolocation data from the response.
                    if let Some(result) = body.get("result") {
                        if let Some(data) = result.get("data") {
                            let location: types::GeolocateData =
                                wick_component::from_value(data.clone()).map_err(|e| {
                                    anyhow::anyhow!("failed to parse response: {}", e)
                                })?;

                            //send stream of outputs to wick
                            outputs.location.send(&location);
                            outputs.status.send(&status);
                        } else {
                            //send stream of errors to wick
                            outputs.status.error("missing data from Pangea api");
                            continue;
                        }
                    } else {
                        //send stream of errors to wick
                        outputs.status.error("missing result from Pangea api");
                        continue;
                    }
                }
            }
        }

        // Let wick know that the response streams are done.
        outputs.status.done();
        outputs.location.done();

        Ok(())
    }
}
