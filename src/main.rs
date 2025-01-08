//use clap::Parser;
use dotenv::dotenv;
use reqwest::Error;
use std::env;
use std::fs;
use std::io;
use std::str;
#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};
/// Simple program to greet a person
/*
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
*/
fn get_stops() -> (String, String) {
    let mut origin = String::new();
    let mut destination = String::new();
    let _ = io::stdin().read_line(&mut origin);
    let _ = io::stdin().read_line(&mut destination);
    (origin, destination)
}
async fn get_request(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    //println!("Status: {}", response.status());

    let body = response.text().await?;
    //println!("Body:\n{}", body);

    Ok(body)
}
fn get_id(content: String) -> String {
    let obj: serde_json::Value = serde_json::from_str(&content).expect("Pech gehabt ;)");
    let stoplocationorcoordlocation: serde_json::Value =
        obj["stopLocationOrCoordLocation"][0].clone();
    let stoplocation: serde_json::Value = stoplocationorcoordlocation["StopLocation"].clone();
    //println!("{}", stoplocation["id"]);
    let mut result = stoplocation["id"].to_string();
    result.pop(); // remove last
    if result.len() > 0 {
        result.remove(0); // remove first
    }

    return result;
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    // let args = Args::parse();
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("env-var API_KEY muss gesetzt sein!\n");
    //Die Haltestellen id bekommt man über location.name
    let (origin_name, destination_name) = get_stops();
    let origin_url = format!(
        "https://www.rmv.de/hapi/location.name?accessId={}&input={}&format=json",
        api_key, origin_name
    );
    let destination_url = format!(
        "https://www.rmv.de/hapi/location.name?accessId={}&input={}&format=json",
        api_key, destination_name
    );
    let origin_content: String = get_request(&origin_url).await?;
    let destination_content: String = get_request(&destination_url).await?;

    // returned die id vom json aus der api request
    let origin_id = get_id(origin_content);
    let destination_id = get_id(destination_content);

    //sucht nach einer Verbindung für zwei Haltestellen
    let url = format!(
        "https://www.rmv.de/hapi/trip?accessId={}&originId={}&destId={}&format=json",
        api_key, origin_id, destination_id
    );
    let result = get_request(&url).await?;
    let _ = fs::write("./new.json", result);

    //Infos über eine Haltestelle : let url = format!("https://www.rmv.de/hapi/location.name?accessId={}&input=bad vilbel%20alte frankfurter straße&format=json",api_key);
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.add_row(row!["Haltestelle", "ID"]);
    table.add_row(row![origin_name, origin_id]);
    //table.add_row(row![origin_name, destination]);
    table.add_row(row![destination_name, destination_id]);
    table.printstd();
    Ok(())
}
