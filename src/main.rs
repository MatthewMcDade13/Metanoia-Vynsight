extern crate lmdb;
extern crate hyper;
extern crate hyper_tls;
extern crate actix;
extern crate ciborium;
extern crate scraper;
extern crate clap;
extern crate serde_json;


use common::{ser::{into_json, from_json}, CrawlResult};
use error::spider::SpiderFailure;
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use serde::{Serialize, Deserialize};
use spider_sup::{SpiderStatus};
use std::error::Error;
use clap::{command, arg};
use actix::prelude::*;

use crate::spider::{Spider, SpiderConfig};


mod db;
mod spider;
mod spider_sup;
mod crawl;
mod scrape;
mod common;
mod error;
mod web;

type MainResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const DEFAULT_CONFIG_PATH: &'static str = "./src/pub/config.json";

#[actix::main]
async fn main() -> MainResult<()> {

    let args = command!()
        .arg(arg!(--config <PATH>).required(false).default_value(DEFAULT_CONFIG_PATH))
        .get_matches();
        // .arg(arg!(--blind).required_unless_present("config"))
        // .arg(arg!(--search <TERM>).required_unless_present_any(["config", "blind"]))
        // .arg(arg!(--targets <FILEPATH>).required_unless_present_any(["config", "db"]))
        // .arg(arg!(--db).required_unless_present_any(["targets", "config"]))
        // .arg(arg!(--single).required_unless_present("config"))
        // .get_matches();

    if let Some(cfg_path) = args.get_one::<String>("config") {

        let config = get_config(cfg_path)?;

        let spider_result = run_spider(&config).await?;

        exit_on_ok(&spider_result)
    } else {
        // Bail, we dont have any input
        exit_on_err(&SpiderFailure("No input provided for Crawling".into()))
    }


   
    
}

fn get_config(cfg_path: &str) -> MainResult<SpiderConfig> {
    let mut config = json_from_file!(SpiderConfig, cfg_path)?;
    let tfopt = config.target_file.clone();
    if let Some(targets_file) = tfopt.as_deref() {
        let _ = config.populate_targets_from_file(targets_file)?;
    }
    Ok(config)
}

async fn run_spider(config: &SpiderConfig) -> MainResult<Vec<CrawlResult>> {
    let targets = config.targets.uris().to_vec();
    let spider = Spider::new(&targets);
    let spider_result = spider.run().await?;    
    
    Ok(spider_result)
}

fn exit_on_ok(new_targets: &Vec<CrawlResult>) -> MainResult<()> {
    // TODO :: Save new_targets to DB / Text file for next run.
    println!("Success! new_targets = {:?}", new_targets);
    Ok(())
}

fn exit_on_err(err: &SpiderFailure) -> MainResult<()> {
    let b = Box::new(err.clone()); 
    Err(b)
}