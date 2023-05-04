use core::num;
use std::fmt::Display;
use std::future::{IntoFuture, Future};
use std::panic::AssertUnwindSafe;
use std::sync::{Arc, RwLock};
use std::collections::{HashMap, HashSet};
use std::task::Poll;
use std::time::Duration;

use actix::dev::{MessageResponse, OneshotSender};
use actix::prelude::*;
use hyper::http::status;
use serde::{Deserialize, Serialize};
use crate::common::{Kill, DoneCrawl,SpawnCrawler, SpiderDone, GetSpiderStatus, ser::{into_cbor, from_cbor, from_json}, DynResult, CrawlResult};
use crate::crawl::{Crawler, Crawl};
use crate::error::spider::SpiderFailure;
use crate::json_from_file;
use crate::spider_sup::{SpiderSupervisor, SpiderStatus};





pub struct Spider {
    master_target_list: Vec<String>,
    sup_handle: Addr<SpiderSupervisor>,
    poll_every: Duration,
}

impl Spider {

    pub const DEFAULT_POLL_DUR: Duration = Duration::from_secs(10);
    pub const DEFAULT_NUM_CRAWLERS: usize = 8;

   pub fn new(starting_list: &Vec<String>) -> Self {
        Self::with_ncrawlers(starting_list, Self::DEFAULT_NUM_CRAWLERS)
   }

   pub fn with_ncrawlers(starting_list: &Vec<String>, num_crawlers: usize) -> Self {
        let tl = starting_list.to_vec();
        let sup_handle = SpiderSupervisor::with_ncrawlers(&tl, num_crawlers).start();
        Self {
            master_target_list: tl,
            sup_handle,
            poll_every: Self::DEFAULT_POLL_DUR,
        }
   }

   pub fn with_search(starting_list: &Vec<String>, num_crawlers: usize, term: &str) -> Self {
        let tl = starting_list.to_vec();
        let sup_handle = SpiderSupervisor::with_search(&tl, num_crawlers, term).start();
        Self {
            master_target_list: tl,
            sup_handle,
            poll_every: Self::DEFAULT_POLL_DUR
        }
   }

   pub fn with_config(config: &SpiderConfig) -> Self  {
        let targets = &config.targets.uris;
        let poll_every = config.poll_interval_dur();
        let num_crawlers = config.num_crawlers as usize;
        let mut inst = if let Some(search_term) = &config.targets.search_term {
            Self::with_search(targets, num_crawlers, search_term)
        } else { 
            Self::with_ncrawlers(targets,num_crawlers)
        };
        inst.set_poll_interval(poll_every);
        inst

   }

   pub fn with_poll_interval(starting_list: &Vec<String>, poll_every: Duration) -> Self {
        let mut inst = Self::new(starting_list);
        inst.set_poll_interval(poll_every);
        inst
   }

   pub fn set_poll_interval(&mut self, duration: Duration) { self.poll_every = duration; }

   pub async fn poll_status(&self) -> Option<SpiderStatus> {
        let status_result = self.sup_handle.send(GetSpiderStatus).await;
        match &status_result {
            Ok(status) => Some(status.clone()),
            Err(_) => None
        }
    }

    pub async fn run(&self) ->  Result<Vec<CrawlResult>, SpiderFailure> {
        loop {
            tokio::time::sleep(self.poll_every).await;

            let polled = self.poll_status().await;
            if let Some(status) = &polled {
                if let SpiderStatus::Done(new_targets) = status {
                    break Ok(new_targets.to_vec());
                } else if let SpiderStatus::Failed(reason) = status {
                    break Err(SpiderFailure(reason.clone()));
                } else { continue; }
            } else { break Err(SpiderFailure("Unknown Error when polling GetSpiderStatus :: Actor is probably dead".into())) }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiderTargets {
    search_term: Option<String>,
    uris: Vec<String>
}

impl SpiderTargets {
    pub fn search_term(&self) -> Option<&str> { self.search_term.as_deref() }
    pub fn uris(&self) -> &[String] { self.uris.as_slice() }
}

impl Default for SpiderTargets {
    fn default() -> Self {
        Self { search_term: None, uris: Vec::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiderConfig {
    pub target_file: Option<String>,
    pub targets: SpiderTargets,
    pub num_crawlers: u32,
    pub poll_interval_millisec: u64,
}

impl SpiderConfig {


    /// WARN :: This overwrites any targets listed in config.json file (leaves search_term alone)
    /// Returns Result<targets, error> :: A slice of the acquired targets from file if successful, Error otherwise
    pub fn populate_targets_from_file(&mut self, filepath: &str) -> DynResult<&[String]> {
        let targets_str = std::fs::read_to_string(filepath)?;
        self.targets.uris = from_json(&targets_str)?;
        Ok(self.targets.uris.as_slice())
    }
}

impl Default for SpiderConfig {
    fn default() -> Self {
        Self {
            target_file: None,
            targets: SpiderTargets::default(),
            num_crawlers: Spider::DEFAULT_NUM_CRAWLERS as u32,
            poll_interval_millisec: Spider::DEFAULT_POLL_DUR.as_millis() as u64
        }
    }
}

impl SpiderConfig {
    pub const fn poll_interval_dur(&self) -> Duration { Duration::from_millis(self.poll_interval_millisec) }
}
