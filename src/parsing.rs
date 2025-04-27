
use serde_json::Value;

pub fn parse_query(params: Option<String>) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut query = Vec::new();

    if let Some(params) = params {
        for q in params.split('&') {
            let kv: Vec<&str> = q.splitn(2, '=').collect();
            if kv.len() == 2 {
                query.push((kv[0].to_string(), kv[1].to_string()));
            }
        }
    }
    
    Ok(query)
}


pub fn parse_header(params: Vec<String>) -> Result<Vec<(String,String)>, Box<dyn std::error::Error>> {
    
    let mut headers = Vec::new();
    
    for header in params {
        let kv: Vec<&str> = header.splitn(2, ':').collect();
        
        if kv.len() == 2 {
            headers.push((kv[0].to_string(), kv[1].to_string()));
        }
    }
    
    
    Ok(headers)
}


pub fn parse_json(params: Option<String>) -> Result<Option<Value>, Box<dyn std::error::Error>> {

    if let Some(json_str) = params {
        let json_value: Value = serde_json::from_str(&json_str)?;
        Ok(Some(json_value))
    } else {
        Ok(None)
    }
}