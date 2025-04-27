mod args;
mod parsing;
mod request;

use args::Args;
use clap::Parser;
use parsing::parse_json;
use parsing::parse_query;
use parsing::parse_header;
use request::build_request;
use request::Params;
use figlet_rs::FIGfont;
use std::env;

fn print_welcome_message() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("ZURL");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let is_installed = env::args().len() == 1;  

    if is_installed {
        
        print_welcome_message();
    }
    let args = Args::parse();

    println!("{}", args.method);
    println!("{}", args.url);

    let parsed_query = if let Some(query) = args.query {
        let q = parse_query(Some(query))?;
        q
    } else {
        Vec::new()
    };
    

    let parsed_headers = if args.header.is_empty() {
        Vec::new()
    } else {
        let h = parse_header(args.header.clone())?;
        h
    };

    let parsed_json = if let Some(json) = args.json {
        let j = parse_json(Some(json))?;
        j
    } else {
        None
    };
    
    let params = Params {
        method: args.method,
        url: args.url,
        header: parsed_headers,
        query: parsed_query,
        json: parsed_json, // Option<Value>
    };
    
    build_request(params).await?;
    

    Ok(())
}
