use std::fmt::Display;

use actix::prelude::*;

use crate::spider::SpiderSupervisor;



#[derive(Message)]
#[rtype(result = "()")]
pub struct Crawl(String);

impl Crawl {
    pub const fn target(&self) -> &str { self.0.as_str() }
}

type ParentSpider = Addr<SpiderSupervisor>;
pub struct Crawler(ParentSpider);

impl Crawler {
    pub const fn parent(&self) -> ParentSpider { self.0 }
}

impl Actor for Crawler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Crawler started, Address: {:?}",  ctx.address());
    }
}


impl Handler<Crawl> for Crawler {
    type Result = ();

    fn handle(&mut self, msg: Crawl, ctx: &mut Self::Context) -> Self::Result {
        // Navigate to given URL
        let url = &msg.0;

        Ok("".into())
    }
}
