extern crate lmdb;
extern crate hyper;
extern crate hyper_tls;
extern crate actix;

use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use std::error::Error;
use actix::prelude::*;


mod db;
mod spider;
mod crawl;
mod scrape;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


#[tokio::main]
async fn main() -> Result<()> {
    // This is where we will setup our HTTP client requests.
    
    

    Ok(())
}