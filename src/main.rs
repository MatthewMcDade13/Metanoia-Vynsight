extern crate lmdb;

use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use std::error::Error;

mod db;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    // This is where we will setup our HTTP client requests.

    Ok(())
}