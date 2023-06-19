use std::io::Cursor;

use actix::{Addr, Message};
use serde::{Serialize, Deserialize};
use crate::crawl::{Crawler};
use crate::error::spider::CrawlError;
use crate::web::Target;
use crate::spider_sup::{SpiderStatus};

#[macro_export]
macro_rules! json_from_file {
    ($output_type:ty, $filepath:ident) => {{
        use crate::common::{DynResult, ser::from_json};
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

pub type HyperClient = hyper::client::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>;



#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Kill;

#[derive(Debug, Message, Clone)]
#[rtype(result = "()")]
pub struct DoneCrawl {
    pub result: Result<Target, CrawlError>,
    pub sender: Addr<Crawler>
}

#[derive(Debug, Clone)]
pub struct CrawlResult(pub hyper::http::Uri, pub Target);

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
#[allow(dead_code)]
pub mod ser {
    use super::*;

    pub fn into_json<'a, T: 'a>(value: &T)  -> DynResult<String>
        where T: Serialize + Deserialize<'a> {
            
            let json = serde_json::ser::to_string_pretty(value)?;
    
            Ok(json)
    }
    
    pub fn from_json<T>(json: &str) -> DynResult<T> 
        where for<'a> T: Deserialize<'a> {
        let value = serde_json::de::from_str::<T>(&json)?;
        
        Ok(value)
    }

}    

