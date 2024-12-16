
pub struct TripRequest {
    pub api_key: String,
    pub origin: &'static str,
    pub destination: &'static str,
    pub format: &'static str,
}

/// Builds RMV trip URL from TripRequest.
pub fn build_trip_url(request: &TripRequest) -> String {
    format!(
        "https://www.rmv.de/hapi/trip?accessId={}&originId={}&destId={}&format={}",
        request.api_key, request.origin, request.destination, request.format
    )
}

/// Validates CLI arguments 
pub fn validate_args(
    haltestelle: &str,
    arrival: &str,
    departure: &str,
) -> Result<(), &'static str> {
    if haltestelle.is_empty() || arrival.is_empty() || departure.is_empty() {
        return Err("One field is empty.");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_trip_url() {
        let req = TripRequest {
            api_key: "TEST_KEY".to_string(),
            origin: "3000507@B=1@p=1733939625@",
            destination: "3002404@B=1@p=1733939625@",
            format: "json",
        };
        let url = build_trip_url(&req);
        // Read about this 2 asserts per function thing from TigerStyle 
        assert!(url.contains("TEST_KEY"), "URL should contain the API key.");
        assert!(url.contains("trip?accessId="), "URL should contain 'trip?accessId='.");
    }

    #[test]
    fn test_validate_args() {
        // Check that valid arguments pass.
        let valid = validate_args("Frankfurt", "10:00", "10:30");
        assert!(valid.is_ok(), "Expected Ok for valid inputs.");
        
        // Check that empty argument fails.
        let invalid = validate_args("", "10:00", "10:30");
        assert!(invalid.is_err(), "Expected Err for empty haltestelle.");
    }
}
