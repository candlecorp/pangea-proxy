#Pangea Proxy
This is a project to show how to use Wick to create a HTTP proxy that will use the Pangea Cloud api to enrich the request with the user's location and then forward the request to the appropriate server.

This will allow any existing service to be location aware without having to change the source code of service itself. The location information is added to the request as a header and can be logged for research purposes.

## How to use

Clone this repo

get a token from the Pangea Cloud API

Update the `docker-compose.yml` file with the token `- PANGEA_TOKEN=<YOURTOKEN>`

`docker-compose up -d`

`curl -X POST -v 'localhost:8080/post?show_env=1'  -H "x-forwarded-for: 11.108.2.98"`

This will return with the location information added to the request as a header.

```
{
  "args": {
    "show_env": "1"
  },
  "data": "",
  "files": {},
  "form": {},
  "headers": {
    "Accept": "*/*",
    "Host": "httpbin",
    "User-Agent": "curl/7.88.1",
    "X-Forwarded-For": "11.108.2.98, 172.23.0.1",
    "X-Pangea-Country-Code": "us"
  },
  "json": null,
  "origin": "11.108.2.98, 172.23.0.1",
  "url": "http://httpbin/post?show_env=1"
}
```

This request header can then be logged or used as part of the application logic.

## How it works:

This is built on the Wick application framework. This allows for modular solutions to be built and reused.

The `proxy.wick` and the `middleware.wick` files contain all the configuration for the header enrichment. More enrichment actions can easily be added to the `middleware.wick` file.

For documentation on Wick, head to https://candle.dev/docs

## Complete reusability

The pangea functionality is completely separated from the HTTP flow. To show it, you can just run the pangea api directly from the CLI.

`curl -sSL sh.wick.run | bash -s -- nightly` to install Wick.
`export PANGEA_TOKEN="<MYTOKEN>"`
`wick invoke registry.candle.dev/pangea/pangea:0.2.0 ip_geolocate  -- --ip="91.101.44.98"`
returns:

```
{"payload":{"value":{"city":"glostrup","country":"Kingdom Of Denmark","country_code":"dk","latitude":55.66999816894531,"longitude":12.399999618530273,"postal_code":"2600"}},"port":"geolocation"}`
```
