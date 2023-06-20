mod wick {
    wick_component::wick_import!();
}
use wick::*;

//make a struct for response body

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
        //import the config "with" variables
        let config = ctx.root_config();

        while let Some(ip) = ip.next().await {
            let ip = propagate_if_error!(ip, outputs, continue);

            println!("ip: {}", ip);

            let (mut get_geolocate_response, mut get_geolocate_response_body) = ctx
                .provided()
                .pangea_api
                .ip_geolocate(once(ip), once(config.token.clone()))?;

            while let (Some(get_geolocate_response), Some(get_geolocate_response_body)) = (
                get_geolocate_response.next().await,
                get_geolocate_response_body.next().await,
            ) {
                let response = propagate_if_error!(get_geolocate_response, outputs, continue);
                let body = propagate_if_error!(get_geolocate_response_body, outputs, continue);

                println!("response: {:?}", response);
                println!("body: {:?}", body);

                if response.status != types::http::StatusCode::Ok {
                    outputs.status.error(format!(
                        "Pangea api returned status code {}",
                        response.status
                    ));
                    continue;
                }

                let status = body.get("status");
                let status = match status {
                    Some(status) => status.as_str().map(|s| s.to_string()),
                    _ => None,
                };

                match &status {
                    Some(status) => println!("status: {}", status),
                    _ => {
                        outputs.status.error(format!("IP Not found"));
                        continue;
                    }
                };

                let status = status.unwrap();

                if status != "Success" {
                    outputs
                        .status
                        .error(format!("Pangea api returned status {}", status));
                    continue;
                } else {
                    println!("status: {}", status);

                    if let Some(result) = body.get("result") {
                        if let Some(data) = result.get("data") {
                            let location: types::GeolocateData =
                                wick_component::from_value(data.clone()).map_err(|e| {
                                    anyhow::anyhow!("failed to parse response: {}", e)
                                })?;

                            outputs.location.send(&location);

                            outputs.status.send(&status.to_string());
                        } else {
                            outputs.status.error("missing data from Pangea api");
                            continue;
                        }
                    } else {
                        outputs.status.error("missing result from Pangea api");
                        continue;
                    }
                }
            }
        }
        outputs.status.done();
        outputs.location.done();
        Ok(())
    }
}

// - name: GeolocateData
// kind: wick/type/struct@v1
// fields:
//   - name: country
//     type: string
//   - name: city
//     type: string
//   - name: latitude
//     type: f32
//   - name: longitude
//     type: f32
//   - name: postal_code
//     type: string
//   - name: country_code
//     type: string
