// This file is part of the project IntBridge
// Copyright (c) 2023 - Joaquin 'ShyanJMC' Manuel Crespo
// https://github.com/ShyanJMC/
// https://git.shyanjmc.com

//  log allows use it to trace every state
use log::{debug, error, log_enabled, info, Level};

// Chrono used to see localtime in UTC and others times 
use chrono::prelude::*;

// Serde means; "serialize" and "deserialize
// Is used for any data
use serde::{Serialize,Deserialize};

// Rocket for use in HTTPS requests
#[macro_use] extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
	env_logger::init();

	let version = "0.1.0";
	let localtime: DateTime<Local> = Local::now();
	
    // To print log information you just execute with RUST_LOG=info
    // warn
    // info
    // debug
    // trace
    info!("Started version {version} at {localtime}");

    // Overwrite the default configuration of webserver
    let overwrite_default_configuration = rocket::Config::figment()
    										.merge(("port", 8443))
    										.merge(("ident", "IntBridge"))
    										.merge(("workers", 20))
    										.merge(("address", "0.0.0.0"));
    										

	info!("Applied custom configuration");

    // Start the web server and add index function as route
    // Instead use "rocket::build()" which start with default configuration
    // we use custom to use above configuration
    // Each mount adds a route
    let _webserver = rocket::custom(overwrite_default_configuration)
    				.mount("/", routes![index])
    				.launch()
    				.await?;

	info!("Started rocket");
	
    Ok(())
}

// This attribute sets the route
#[get("/")]
fn index() -> &'static str {
	""
}

#[get("/_")]
fn empty() -> &'static str {
	""
}
