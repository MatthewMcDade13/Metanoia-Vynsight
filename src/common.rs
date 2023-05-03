use std::path::Path;
use std::sync::{Arc, RwLock};
use std::io::Cursor;

use actix::{Addr, Message};
use serde::{Serialize, Deserialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::crawl::{Crawler, CrawlError};
use crate::spider_sup::{SpiderStatus};
use crate::web::Target;

#[macro_export]
macro_rules! json_from_file {
    ($output_type:ty, $filepath:ident) => {{
        use crate::common::{DynResult, from_json};
            (|| -> DynResult<$output_type> {
                 let json = std::fs::read_to_string($filepath)?;
                 let value = from_json::<$output_type>(&json)?;
                 Ok(value)
             })()
    }};
    ($output_type:ty, $filepath:literal) => {{
        use crate::common::{DynResult, from_json};
            (|| -> DynResult<$output_type> {
                 let json = std::fs::read_to_string($filepath)?;
                 let value = from_json::<$output_type>(&json)?;
                 Ok(value)
             })()
    }};
}


#[derive(Debug, Clone)]
pub enum ConnectionErrorType {
    HTTP,
    HTTPS,
    TLS,
    FTP,
    SFTP,
    SSH,
    WebSocket,
}


#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Kill;

#[derive(Message)]
#[rtype(result = "()")]
pub struct DoneCrawl {
    result: Result<Target, CrawlError>,
    sender: Addr<Crawler>
}

impl DoneCrawl {
    pub const fn result(&self) -> &Result<Target, CrawlError> { &self.result }
    pub fn sender(&self) -> Addr<Crawler> { self.sender.clone() } 
}

pub type URL = String;
#[derive(Debug, Clone)]
pub struct CrawlResult(pub URL, pub Target);

#[derive(Message)]
#[rtype(result = "()")]
pub struct SpawnCrawler;

type TargetUri = String;
#[derive(Message)]
#[rtype(result = "()")]
pub struct Crawl(pub TargetUri);

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct SpiderDone(pub Vec<CrawlResult>);


#[derive(Debug, Message)]
#[rtype(result = "SpiderStatus")]
pub struct GetSpiderStatus;

pub type DynResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;


pub fn into_cbor<'a, T: 'a>(value: &T)  -> DynResult<Vec<u8>>
    where T: Serialize + Deserialize<'a> {
        
        let mut value_buffer: Vec<u8> = Vec::new();
        ciborium::ser::into_writer(value, &mut value_buffer)?;

        Ok(value_buffer.clone())
}

pub fn from_cbor<'a, T: 'a>(cbor: &[u8]) -> DynResult<T> 
    where T: Deserialize<'a> {

    let value = ciborium::de::from_reader::<T, _>(Cursor::new(cbor))?;
    Ok(value)
}

pub fn into_json<'a, T: 'a>(value: &T)  -> DynResult<String>
    where T: Serialize + Deserialize<'a> {
        
        let json = serde_json::ser::to_string_pretty(value)?;

        Ok(json)
}

pub fn from_json<'a, T>(json: &'a str) -> DynResult<T> 
    where T: Deserialize<'a> {
    let value = serde_json::de::from_str::<T>(&json)?;
    
    Ok(value)
}

