// Copyright 2020 Oxide Computer Company
//! Example use of Dropshot.

use dropshot::endpoint;
use dropshot::ApiDescription;
use dropshot::ConfigDropshot;
use dropshot::ConfigLogging;
use dropshot::ConfigLoggingLevel;
use dropshot::HttpError;
use dropshot::HttpResponseOk;
use dropshot::HttpServerStarter;
use dropshot::RequestContext;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;

progenitor::generate_api!(
    spec = "spec.json", // The OpenAPI document
    replace = { Thing = Thing }
);

#[tokio::main]
async fn main() -> Result<(), String> {
    // We must specify a configuration with a bind address.  We'll use 127.0.0.1
    // since it's available and won't expose this server outside the host.  We
    // request port 0, which allows the operating system to pick any available
    // port.
    let config_dropshot: ConfigDropshot = Default::default();

    // For simplicity, we'll configure an "info"-level logger that writes to
    // stderr assuming that it's a terminal.
    let config_logging =
        ConfigLogging::StderrTerminal { level: ConfigLoggingLevel::Info };
    let log = config_logging
        .to_logger("apitest")
        .map_err(|error| format!("failed to create logger: {}", error))?;

    // Build a description of the API.
    let mut api = ApiDescription::new();
    api.register(get_thing).unwrap();

    api.openapi("apitest", "1.0.0").write(&mut std::io::stdout()).unwrap();

    // Set up the server.
    let server = HttpServerStarter::new(&config_dropshot, api, (), &log)
        .map_err(|error| format!("failed to create server: {}", error))?
        .start();

    let addr = server.local_addr();

    // Wait for the server to stop.  Note that there's not any code to shut down
    // this server, so we should never get past this point.
    let task = tokio::spawn(server);

    // On the client, make a request.
    let client = Client::new(&format!("http://{}:{}", addr.ip(), addr.port()));
    println!("\nclient got: {:?}", client.get_thing().await);
    task.abort();
    Ok(())
}

// HTTP API interface

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Thing {
    a: String,
    #[schemars(skip)]
    b: String,
}

#[endpoint {
    method = GET,
    path = "/thing",
}]
async fn get_thing(
    _rqctx: RequestContext<()>,
) -> Result<HttpResponseOk<Thing>, HttpError> {
    Ok(HttpResponseOk(Thing { a: "ok".to_string(), b: "oops".to_string() }))
}
