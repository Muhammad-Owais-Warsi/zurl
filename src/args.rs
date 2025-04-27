use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "zurl", version = "0.1", author = "Muhammad Owais Warsi")]
pub struct Args {
    
    pub method: String,
    
    #[arg(short = 'j', long = "json")]
    pub json: Option<String>,
    
    #[arg(short = 'q', long = "query")]
    pub query: Option<String>,
    
    #[arg(short = 'H', long = "header")]
    pub header: Vec<String>,
    
    pub url: String,
}
