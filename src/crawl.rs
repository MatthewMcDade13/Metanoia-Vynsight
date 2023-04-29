use std::fmt::Display;

use actix::prelude::*;

use crate::{spider_sup::SpiderSupervisor, common::ConnectionErrorType};



#[derive(Message)]
#[rtype(result = "()")]
pub struct Crawl(pub String);


impl Crawl {
    pub fn target(&self) -> &str { self.0.as_str() }
}

pub struct Crawler {
    parent: Addr<SpiderSupervisor>,
}

impl Crawler {
    pub const fn new(parent: Addr<SpiderSupervisor>) -> Self {     
        Self { parent }
    }
    pub fn start_with(parent: Addr<SpiderSupervisor>) -> Addr<Self> {
        Self::new(parent).start()
    }
    pub fn parent(&self) -> Addr<SpiderSupervisor> { self.parent.clone() }
}


impl Actor for Crawler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Crawler started, Address: {:?}",  ctx.address());
    }
}

impl Supervised for Crawler {}


impl Handler<Crawl> for Crawler {
    type Result = ();

    fn handle(&mut self, msg: Crawl, ctx: &mut Self::Context) -> Self::Result {
        let url = &msg.0;
        
        // Navigate to given URL
        todo!();

        // Parse HTML and get links
        todo!();
    }
}

type Reason = String;
#[derive(Debug, Clone)]
pub enum CrawlErrorInfo {
    Connection(ConnectionErrorType, Reason),
    Parse(Reason)
}

pub struct CrawlError {
    uri: String,
    info: CrawlErrorInfo
}

impl CrawlError {
    pub fn at_uri(&self) -> &str { self.uri.as_str() }
    pub const fn info(&self) -> &CrawlErrorInfo { &self.info }
}