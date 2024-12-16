use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    ///  haltestelle
    #[arg(long)]
    pub haltestelle: String,

    /// Öffi ID
    #[arg(short, long)]
    pub id: i32,

    /// Arrival time 
    #[arg(short, long)]
    pub arrival: String,

    /// Departure time 
    #[arg(short, long)]
    pub departure: String,
}


pub fn parse_cli_args() -> Args {
    Args::parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cli_args_with_empty() {
      
        let test_args = Args {
            haltestelle: "".to_string(),
            id: 0,
            arrival: "".to_string(),
            departure: "".to_string(),
        };
        assert_eq!(test_args.haltestelle, "");
        assert_eq!(test_args.id, 0);
    }

    #[test]
    fn test_parse_cli_args_with_values() {
        let test_args = Args {
            haltestelle: "Frankfurt".to_string(),
            id: 123,
            arrival: "10:00".to_string(),
            departure: "10:30".to_string(),
        };
        assert_eq!(test_args.haltestelle, "Frankfurt");
        assert_eq!(test_args.arrival, "10:00");
    }
}
