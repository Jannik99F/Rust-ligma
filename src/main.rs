use clap::Parser;
#[macro_use] extern crate prettytable;
use prettytable::{Table,format};


/// Simple program to greet a person

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    haltestelle: String,

    #[arg(short,long)]
    id: f32,

    #[arg(short,long)]
    arrival: String,

    #[arg(short,long)]
    departure: String,
}

fn main() {
    let args = Args::parse();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.add_row(row!["Haltestelle","Bus/Bahn ID","Ankfunft","Abfahrt"]);
    table.add_row(row![args.haltestelle ,args.id ,args.arrival,args.departure]);
    table.printstd();
    }
