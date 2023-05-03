use core::num;
use std::fmt::Display;
use std::future::{IntoFuture, Future};
use std::sync::{Arc, RwLock};
use std::collections::{HashMap, HashSet};
use std::task::Poll;

use actix::dev::{MessageResponse, OneshotSender};
use actix::prelude::*;
use hyper::http::status;
use crate::common::{ Kill, DoneCrawl,SpawnCrawler, SpiderDone, GetSpiderStatus, CrawlResult};
use crate::crawl::{Crawler, Crawl};
use crate::spider::Spider;

struct CrawlerQueue {
    idle: Vec<Addr<Crawler>>,
    active: HashMap<Addr<Crawler>, String>
}

impl CrawlerQueue {
    fn with_capacity(n: usize) -> Self {
        Self {
            idle: Vec::with_capacity(n),
            active: HashMap::with_capacity(n)
        }
    }

    fn clear(&mut self) {
        self.idle.clear();
        self.active.clear();
    }
}

impl Default for CrawlerQueue {
    fn default() -> Self {
        Self {
            idle: Vec::new(),
            active: HashMap::new()
        }
    }
}

type TargetList = Vec<String>;
pub struct SpiderSupervisor {
    targets: Vec<String>,
    targets_vistited: HashSet<String>, 
    results: Vec<CrawlResult>,
    crawlers: CrawlerQueue,
    num_crawlers: usize,
    status: SpiderStatus,
    search_term: Option<String>,
    // addr: Addr<Self>,
}

impl Default for SpiderSupervisor {
    fn default() -> Self {
        Self {
            targets: Vec::new(),
            targets_vistited: HashSet::new(),
            crawlers: CrawlerQueue::with_capacity(Spider::DEFAULT_NUM_CRAWLERS),
            num_crawlers: Spider::DEFAULT_NUM_CRAWLERS,
            status: SpiderStatus::PendingStart,
            search_term: None,
            results: Vec::new()
        }
    }
}

impl SpiderSupervisor {
    pub const fn targets(&self) -> &TargetList { &self.targets }
    pub const fn ncrawlers(&self) -> usize { self.num_crawlers }

    pub fn new(targets: &Vec<String>) -> Self {
        Self::with_ncrawlers(targets, Spider::DEFAULT_NUM_CRAWLERS)
    }

    pub fn with_ncrawlers(targets: &Vec<String>, num_crawlers: usize) -> Self {
        let ntargets = targets.len();
        Self {
            targets: targets.to_vec(),
            targets_vistited: HashSet::with_capacity(ntargets),
            crawlers: CrawlerQueue::with_capacity(num_crawlers),
            num_crawlers,
            status: SpiderStatus::PendingStart,
            search_term: None,
            results: Vec::new()
        }
    }

    pub fn with_search(targets: &Vec<String>, num_crawlers: usize, term: &str) -> Self {
        let mut inst = Self::with_ncrawlers(targets, num_crawlers);
        inst.search_term = Some(term.to_string());
        inst
    }

    fn setup_once(&mut self, ctx: &mut Context<SpiderSupervisor>) {
        println!("Started SpiderSupervisor {:?}", ctx.address());

        self.status = SpiderStatus::Running;

        let addr = ctx.address();
        let n_crawlers = self.num_crawlers;

        for _ in 0..n_crawlers {
            let parent = addr.clone();
            let crawler = Crawler::new(parent.clone()).start();//Supervisor::start(move |_| Crawler::new(parent.clone()));

            if let Some(uri) = &self.targets.pop() {

                crawler.do_send(Crawl(uri.to_string()));
                self.crawlers.active.insert(crawler, uri.to_string());             

            } else {
                self.crawlers.idle.push(crawler);
            }

        }
    }
}



impl actix::Supervised for SpiderSupervisor {
    fn restarting(&mut self, ctx: &mut Self::Context) {
        println!("Restarting Spider Actor thread: {:?}", ctx.address());
        self.crawlers.clear();
    }
}

impl Handler<DoneCrawl> for SpiderSupervisor {
    type Result = ();

    fn handle(&mut self, msg: DoneCrawl, ctx: &mut Self::Context) -> Self::Result {
        let crawl_result = msg.result();
        match crawl_result {
            Ok(target) => {
                for uri in target.child_links() {
                    if !self.targets_vistited.contains(uri) {
                        self.targets.push(uri.clone());
                    }
                }

                self.results.push(CrawlResult(target.uri().to_string(), target.clone()))
            },
            Err(err) => {
                println!("Error Crawling URL: {}, Error: {:?}", err.at_uri(), err.info())
            }
        }

        let sender = msg.sender();
        if self.targets.len() > 0 {
            
            while let Some(crawler) = &self.crawlers.idle.pop() {
                if let Some(target_uri) = &self.targets.pop() {
                    self.crawlers.active.insert(crawler.clone(), target_uri.to_string());
                    crawler.do_send(Crawl(target_uri.to_string()))
                } else { 
                    if self.crawlers.active.len() == 0 {
                        ctx.notify(SpiderDone(self.results.to_vec()));
                    }
                }
            }

        } else if self.crawlers.active.len() == 0 {
            ctx.notify(SpiderDone(self.results.to_vec()));
        } else {            
            self.crawlers.active.remove(&sender);
            self.crawlers.idle.push(sender.clone());
        }
    }
}


impl Handler<SpiderDone> for SpiderSupervisor {
    type Result = ();

    fn handle(&mut self, msg: SpiderDone, ctx: &mut Self::Context) {
        let addr = ctx.address();
        let SpiderDone(targets) = msg;
        self.status = SpiderStatus::Done(targets.to_vec());
    }
}

impl Handler<Kill> for SpiderSupervisor {
    type Result = ();

    fn handle(&mut self, _: Kill, ctx: &mut Self::Context) {
        ctx.stop();
    }
}

impl Handler<GetSpiderStatus> for SpiderSupervisor {
    type Result = SpiderStatus;

    fn handle(&mut self, _: GetSpiderStatus, ctx: &mut Self::Context) -> Self::Result {
        self.status.clone()
    }
}



impl Actor for SpiderSupervisor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {

        // Dont need to setup if we already have before
        match self.status {
            SpiderStatus::PendingStart => self.setup_once(ctx),
            _ => {
                println!("SpiderSupervisor::started() called & SpiderSupervisor::SpiderStatus != SpiderStatus::PendingStart. We are probably restarting. aborting started()");
            }
        }

    }
}


#[derive(Debug, Clone)]
pub enum SpiderStatus {
    Running, Done(Vec<CrawlResult>), Failed(String), PendingStart
}

impl<A, M> MessageResponse<A, M> for SpiderStatus 
where     
    A: Actor,
    M: Message<Result = Self>, 
{
    fn handle(self, ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}


