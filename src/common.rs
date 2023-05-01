use std::sync::{Arc, RwLock};
use std::io::Cursor;

use actix::{Addr, Message};
use serde::{Serialize, Deserialize};

use crate::crawl::{Crawler, CrawlError};
use crate::spider_sup::{SpiderStatus};

#[derive(Debug, Copy, Clone)]
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
    result: Result<Vec<String>, CrawlError>,
    sender: Addr<Crawler>
}

impl DoneCrawl {
    pub const fn result(&self) -> &Result<Vec<String>, CrawlError> { &self.result }
    pub fn sender(&self) -> Addr<Crawler> { self.sender.clone() } 
}
// impl DoneCrawl {
//     pub fn new()
// }


#[derive(Message)]
#[rtype(result = "()")]
pub struct SpawnCrawler;

type TargetUri = String;
#[derive(Message)]
#[rtype(result = "()")]
pub struct Crawl(pub TargetUri);

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct SpiderDone(pub Vec<String>);


#[derive(Debug, Message)]
#[rtype(result = "SpiderStatus")]
pub struct GetSpiderStatus;

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;


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