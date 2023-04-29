extern crate lmdb;
extern crate hyper;
extern crate hyper_tls;
extern crate actix;
extern crate ciborium;

use common::{into_cbor, from_cbor};
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use serde::{Serialize, Deserialize};
use spider_sup::{SpiderFailure, SpiderStatus};
use std::error::Error;
use actix::prelude::*;

use crate::spider::{Spider};


mod db;
mod spider;
mod spider_sup;
mod crawl;
mod scrape;
mod common;

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    x: u32, y: u32
}


// #[actix::main]
fn main() -> MainResult<()> {

    let p = Point {x: 1, y: 2};
    println!("START: {:?}", p);
    let d = into_cbor(&p).unwrap();
    println!("INTO_CBOR: {:?}", d);
    let r = from_cbor::<Point>(&d).unwrap();
    println!("FROM_CBOR: {:?}", r);

    Ok(())
    // let spider = Spider::new(vec!["".into()]);
/*  */
    // let spider_result = spider.run().await;    
    
    // match &spider_result {
    //     Ok(new_targets) => exit_on_ok(new_targets),
    //     Err(err) => exit_on_err(err),
    // }

}


fn exit_on_ok(new_targets: &Vec<String>) -> MainResult<()> {
    // TODO :: Save new_targets to DB / Text file for next run.
    println!("Success! new_targets = {:?}", new_targets);
    Ok(())
}

fn exit_on_err(err: &SpiderFailure) -> MainResult<()> {
    let b = Box::new(err.clone()); 
    Err(b)
}