use colored::*;
use reqwest::{Client, Method};
use serde_json::Value;

pub struct Params {
    pub method: String,
    pub url: String,
    pub header: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub json: Option<Value>,
}
 
pub async fn build_request(params: Params) -> Result<(), Box<dyn std::error::Error>> {
    let method = params.method.parse::<Method>()?;
    let mut url = params.url;
    
    let client: Client = Client::new();

    if url.starts_with(":") {
        url = format!("http://localhost{}", &url[..]);
    }
    
    let mut request = client.request(method, &url);

    if let Some(json) = params.json {
        request = request.header("Content-Type", "application/json");
        let j = Some(json);
        request = request.json(&j);
    }

    if !params.query.is_empty() {
        request = request.query(&params.query);
    }

    if !params.header.is_empty() {
        for (key, value) in &params.header {
            request = request.header(key, value);
        }
    }

    let response = request.send().await?;

    println!("{}", format!("Status: {}", response.status()).green());

    let body = response.text().await?;
    if let Ok(json_body) = serde_json::from_str::<Value>(&body) {
        let pretty_json = serde_json::to_string_pretty(&json_body)?;
        println!("\nResponse Body:\n{}", pretty_json);
    } else {
        println!("\nResponse Body (Not JSON):\n{}", body);
    }

    Ok(())
}
