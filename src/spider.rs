use std::fmt::Display;
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{RwLock};

use actix::prelude::*;
use crate::crawl::Crawler;

pub struct Supervisor {
    root: Addr<Spider>
}

impl Actor for Supervisor {
    type Context = Context<Self>;
}


#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Kill;

type SpiderActorBucketRef = Arc<RwLock<Vec<Addr<Crawler>>>>;
struct SpiderActorBucket(SpiderActorBucketRef);

impl SpiderActorBucket {
    fn as_ref(&self) -> SpiderActorBucketRef {
        self.0.clone()
    }
}

impl Default for SpiderActorBucket {
    fn default() -> Self {
        Self(Arc::new(RwLock::const_new(Vec::new())))
    }
}


pub struct Spider {
    starting_list: Option<Vec<String>>,
    max_crawlers: u32,
    max_crawler_bucket_size: u32,
    actor_bucket: SpiderActorBucket,
}

impl Spider {
   fn new(max_crawlers: u32, starting_list: Option<Vec<String>>) -> Self {
    Self {
        starting_list,
        max_crawlers,
        max_crawler_bucket_size: max_crawlers / 2,
        actor_bucket: SpiderActorBucket::default()
    }
   }
}

impl Default for Spider {
    fn default() -> Self {
        Self {
            starting_list: None,
            max_crawlers: 8,
            max_crawler_bucket_size: 4,
            actor_bucket: SpiderActorBucket::default()
        }
    }
}

impl Actor for Spider {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        if let Some(start_urls) = &self.starting_list {
            let exec = async { 
                 
                let bucket_lock = self.actor_bucket.0.as_ref();
                    {
                        let mut cs = bucket_lock.write().await;
                        for url in start_urls {
                            let addr = Crawler::new(url).start();
                            cs.push(addr);
                        }
                    } 

            };
        }
    }
}


// To use actor with supervisor actor has to implement `Supervised` trait
impl actix::Supervised for Spider {
    fn restarting(&mut self, ctx: &mut Self::Context) {
        
        let addr = ctx.address();
        println!("Restarting Actor thread: {:?}", addr)
    }
}

impl Handler<Kill> for Spider {
    type Result = ();

    fn handle(&mut self, _: Kill, ctx: &mut Context<Spider>) {
        ctx.stop();
    }
}


