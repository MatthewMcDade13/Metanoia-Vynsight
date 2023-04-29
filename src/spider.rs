use core::num;
use std::fmt::Display;
use std::future::{IntoFuture, Future};
use std::sync::{Arc, RwLock};
use std::collections::{HashMap, HashSet};
use std::task::Poll;
use std::time::Duration;

use actix::dev::{MessageResponse, OneshotSender};
use actix::prelude::*;
use hyper::http::status;
use serde::{Deserialize, Serialize};
use crate::common::{Kill, DoneCrawl,SpawnCrawler, SpiderDone, GetSpiderStatus, into_cbor, from_cbor};
use crate::crawl::{Crawler, Crawl};
use crate::spider_sup::{SpiderSupervisor, SpiderStatus, SpiderFailure};





pub struct Spider {
    master_target_list: Vec<String>,
    sup_handle: Addr<SpiderSupervisor>,
    poll_every: Duration,
}

impl Spider {

    pub const DEFAULT_POLL_DUR: Duration = Duration::from_secs(10);

   pub fn new(starting_list: Vec<String>) -> Self {
       let tl = starting_list.to_vec();
        let sup_handle = SpiderSupervisor::new(tl).start();//Supervisor::start(|_| SpiderSupervisor::new(tl));
        Self {
            master_target_list: starting_list,
            sup_handle,
            poll_every: Self::DEFAULT_POLL_DUR
        }
   }

   pub fn with_poll_interval(starting_list: Vec<String>, poll_every: Duration) -> Self {
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

    pub async fn run(&self) ->  Result<Vec<String>, SpiderFailure> {
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

