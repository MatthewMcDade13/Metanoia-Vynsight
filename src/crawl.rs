use std::fmt::Display;

use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "std::io::Result<String>")]
pub struct Crawl(String);

pub struct Crawler {
    uri: String
}


impl Actor for Crawler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Crawler started, Address: {:?}",  ctx.address());
    }
}

impl Crawler {
    pub fn new(uri: &str) -> Self {
        Self { uri: uri.into() }
    }
}

impl Handler<Crawl> for Crawler {
    type Result = std::io::Result<String>;

    fn handle(&mut self, msg: Crawl, ctx: &mut Self::Context) -> Self::Result {
        // Navigate to given URL
        let url = &msg.0;

        Ok("".into())
    }
}