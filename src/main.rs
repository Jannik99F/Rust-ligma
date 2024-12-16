mod cli;
mod network;
mod domain;

use crate::cli::parse_cli_args;
use crate::domain::{TripRequest, build_trip_url, validate_args};
use crate::network::fetch_data;

use dotenv::dotenv;
use std::env;
use reqwest::Error;
use prettytable::{format, Table, row};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Parse CLI args
    let args = parse_cli_args();

    // Load environment variables
    if dotenv().is_err() {
        eprintln!("Warning: .env file could not be loaded. Default environment variables will be used.");
    }

    // Get API_KEY from environment
    let api_key = env::var("API_KEY").unwrap_or_else(|_| {
        eprintln!("Error: API_KEY environment variable is not set. Exiting.");
        std::process::exit(1);
    });

    // Validate CLI arguments
    if let Err(msg) = validate_args(&args.haltestelle, &args.arrival, &args.departure) {
        eprintln!("Error: {}", msg);
        std::process::exit(1);
    }

    // Prepare the TripRequest (data-oriented approach)
    let trip_request = TripRequest {
        api_key,
        origin: "A=1@O=Frankfurt (Main) Nibelungenplatz@X=8693144@Y=50129020@U=80@L=3000507@B=1@p=1733939625@",
        destination: "A=1@O=Bad Vilbel Alte Frankfurter Stra\u{00df}e@X=8728373@Y=50168959@U=80@L=3002404@B=1@p=1733939625@",
        format: "json",
    };

    // Build URL
    let url = build_trip_url(&trip_request);

    // Fetch data
    let _ = fetch_data(&url).await?;

    // Print table
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.add_row(row!["Haltestelle", "Bus/Bahn ID", "Ankunft", "Abfahrt"]);
    table.add_row(row![
        args.haltestelle,
        args.id,
        args.arrival,
        args.departure
    ]);
    table.printstd();

    Ok(())
}
