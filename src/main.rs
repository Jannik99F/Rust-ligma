use clap::Parser;
use reqwest::Error;
use dotenv::dotenv;
use std::env;
#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};

/// Simple program to greet a person

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    haltestelle: String,

    #[arg(short, long)]
    id: i32,

    #[arg(short, long)]
    arrival: String,

    #[arg(short, long)]
    departure: String,
}

 async fn get_request(url:&str) -> Result<(),Error>{
let response = reqwest::get(url).await?;
    println!("Status: {}", response.status());

    let body = response.text().await?;
    println!("Body:\n{}", body);

    Ok(())
 }
 #[tokio::main]
 async fn main() -> Result<(),Error> {
    let args = Args::parse();
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("env-var API_KEY muss gesetzt sein!\n");
    let url = format!("https://www.rmv.de/hapi/location.name?accessId={}&input=frankfurt%20hauptbahnhof&format=json",api_key);
    get_request(&url).await?;
    // let client = reqwest::Client::new();
    // let data = client.get("https://httpbin.org/get")
    //     .await?
    //     .text()
    //     .await?;

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.add_row(row!["Haltestelle", "Bus/Bahn ID", "Ankfunft", "Abfahrt"]);
    table.add_row(row![
        args.haltestelle,
        args.id,
        args.arrival,
        args.departure
    ]);
    table.printstd();
    Ok(())
}
