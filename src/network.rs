use reqwest::Error;

/// Standard error codes for HTTP status codes.
pub async fn fetch_data(url: &str) -> Result<String, Error> {
    let response = reqwest::get(url).await?;
    let status_code = response.status().as_u16();
    
    match status_code {
        200 => {
            let body = response.text().await?;
            println!("Body:\n{}", body);
            Ok(body)
        }
        400 => {
            let msg = "Bad Request: The server could not understand the request.".to_string();
            println!("{}", &msg);
            Ok(msg)
        },
        401 => {
            let msg = "Unauthorized: Access is denied due to invalid credentials.".to_string();
            println!("{}", &msg);
            Ok(msg)
        },
        404 => {
            let msg = "Not Found: The requested resource could not be found.".to_string();
            println!("{}", &msg);
            Ok(msg)
        },
        500..=599 => {
            let msg = "Server Error: The server encountered an error.".to_string();
            println!("{}", &msg);
            Ok(msg)
        },
        _ => {
            let msg = format!("Unexpected status code: {}", status_code);
            println!("{}", &msg);
            Ok(msg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_fetch_data_mock() {
        // Test the fetch_data function with a mock URL.
        let rt = Runtime::new().unwrap();
        let test_url = "https://example.com";
        let result = rt.block_on(fetch_data(test_url));
        
        assert!(result.is_ok(), "Expected Ok result from fetch_data.");
        let body_or_msg = result.unwrap();
        assert!(!body_or_msg.is_empty(), "Response or message should not be empty.");
    }
}
