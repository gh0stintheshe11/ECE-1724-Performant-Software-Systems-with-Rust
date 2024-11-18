use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "curl", about = "A simple HTTP client.")]
// Define Command-Line Arguments: Use structopt to define the CLI structure
struct CurlArgs {
    /// URL to request
    url: String,

    /// HTTP method (GET or POST)
    #[structopt(short = "X", default_value = "GET")]
    method: String,

    /// Data to send in a POST request
    #[structopt(short = "d")]
    data: Option<String>,

    /// JSON data for POST request
    #[structopt(long = "json")]
    json_data: Option<String>,
}

use url::Url;
// Handle URL Parsing and Validation: Use the url crate to validate URLs
// Add specific checks for:
// 1. Valid protocols (e.g., http, https).
// 2. IP addresses.
// 3. Port ranges.
fn validate_url(url: &str) -> Result<Url, String> {
    let parsed_url = Url::parse(url).map_err(|_| "Invalid URL".to_string())?;
    
    // Validate protocol
    match parsed_url.scheme() {
        "http" | "https" => (),
        _ => return Err("Invalid protocol. Only http and https are supported".to_string()),
    }
    
    // Validate port range if specified
    if let Some(port) = parsed_url.port() {
        if port == 0 {
            return Err("Invalid port number".to_string());
        }
    }
    
    Ok(parsed_url)
}

use reqwest::{blocking::Client, Error};

// Make HTTP Requests: Use the reqwest crate for HTTP requests
fn make_request(args: CurlArgs) -> Result<String, String> {
    let client = Client::new();

    match args.method.to_uppercase().as_str() {
        "GET" => {
            let response = client.get(&args.url).send().map_err(|e| e.to_string())?;
            if !response.status().is_success() {
                return Err(format!(
                    "Request failed with status code: {}",
                    response.status()
                ));
            }
            Ok(response.text().map_err(|e| e.to_string())?)
        }
        "POST" => {
            let response = if let Some(data) = args.data {
                client.post(&args.url).body(data).send().map_err(|e| e.to_string())?
            } else if let Some(json_data) = args.json_data {
                client
                    .post(&args.url)
                    .header("Content-Type", "application/json")
                    .body(json_data)
                    .send()
                    .map_err(|e| e.to_string())?
            } else {
                return Err("No data provided for POST request".to_string());
            };
            
            if !response.status().is_success() {
                return Err(format!(
                    "Request failed with status code: {}",
                    response.status()
                ));
            }
            Ok(response.text().map_err(|e| e.to_string())?)
        }
        _ => Err("Unsupported HTTP method".to_string()),
    }
}

// Format JSON Responses: Use serde_json to pretty-print JSON with sorted keys
fn format_json(response_body: &str) -> String {
    match serde_json::from_str::<serde_json::Value>(response_body) {
        Ok(json) => serde_json::to_string_pretty(&json).unwrap_or(response_body.to_string()),
        Err(_) => response_body.to_string(),
    }
}

// Combine everything in the main.rs file
fn main() {
    let args = CurlArgs::from_args();

    println!("Requesting URL: {}", args.url);
    println!("Method: {}", args.method);

    match validate_url(&args.url) {
        Ok(_) => match make_request(args) {
            Ok(body) => println!("Response body:\n{}", format_json(&body)),
            Err(e) => println!("Error: {}", e),
        },
        Err(e) => println!("Error: {}", e),
    }
}