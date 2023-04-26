use std::fmt::Display;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use actix::prelude::*;
use crate::common::{RwLockBucket, ActorBucket, Kill, DoneCrawl, PushTarget, PopTarget, SpawnCrawler};
use crate::crawl::Crawler;

const DEFAULT_MAX_CRAWLERS: usize = 8;


type TargetList = Vec<String>;
pub struct SpiderSupervisor(TargetList);

impl SpiderSupervisor {
    // pub fn start_with(num_crawlers: usize) -> Self {
    //     let addr: Addr<Self> = ...;
    //     addr.
    // }
}

impl Actor for SpiderSupervisor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Starting Spider: {:?}", ctx.address())
    }
}

impl actix::Supervised for SpiderSupervisor {
    fn restarting(&mut self, ctx: &mut Self::Context) {
        println!("Restarting Spider Actor thread: {:?}", ctx.address())
    }
}

impl Handler<DoneCrawl> for SpiderSupervisor {
    type Result = ();

    fn handle(&mut self, msg: DoneCrawl, ctx: &mut Self::Context) -> Self::Result {
    }
}


impl Handler<Kill> for SpiderSupervisor {
    type Result = ();

    fn handle(&mut self, _: Kill, ctx: &mut Self::Context) {
        ctx.stop();
    }
}


impl Handler<SpawnCrawler> for SpiderSupervisor {
    type Result = ();

    fn handle(&mut self, _: SpawnCrawler, ctx: &mut Self::Context) {
        
    }
}

pub struct Spider {
    master_target_list: Vec<String>,
    runner: SystemRunner,
    sup_handle: Addr<SpiderSupervisor>
}

impl Spider {

   pub fn new(sys: SystemRunner, starting_list: Vec<String>) -> Self {
        let master_target_list = &starting_list;
        let sup_handle = {
            let tl = starting_list.to_vec();
            let fut = async { Supervisor::start(|_| SpiderSupervisor(tl)) };
            sys.block_on(fut)
        };
        Self {
            master_target_list: starting_list,
            runner: sys,
            sup_handle
        }
   }

   pub fn start(&self) -> std::io::Result<()> {
    self.runner.run()
   }
}

// impl Default for Spider {
//     fn default() -> Self {
//         Self {
//             target_list: Vec::new(),
//             sup_handle: None
//         }
//     }
// }




