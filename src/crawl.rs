
use actix::prelude::*;
use futures_util::TryFutureExt;
use hyper::{http::response, Uri};
use hyper::{body::HttpBody as _, Client};
use hyper_tls::HttpsConnector;
use scraper::{Html, Selector};

use crate::error::spider::{CrawlError, CrawlErrorInfo};
use crate::scrape::WebScraper;
use crate::web::{Target, GenWebScraper, is_valid_uri_scheme};
use crate::{spider_sup::SpiderSupervisor, common::{HyperClient, DoneCrawl, DynResult}};



#[derive(Message)]
#[rtype(result = "()")]
pub struct Crawl(pub String);

#[allow(dead_code)]
impl Crawl {
    pub fn target(&self) -> &str { self.0.as_str() }
}

pub struct Crawler {
    parent: Addr<SpiderSupervisor>,
    http_client: HyperClient
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
        let uri: Uri = url.parse().unwrap();
        let sender = ctx.address().clone();
        let parent = self.parent.clone();
        let client = self.http_client.clone();

        Arbiter::current().spawn(async move {
            println!("Crawling {}", uri.to_string());
            // TODO :: get_target_body panics on Timeout or bad connection. FIX DIS ISH BOII
            let body = get_target_body(&client, &uri).await.unwrap();
            let target_result = parse_target_body(&body);

            parent.do_send(DoneCrawl {
                result: Ok(Target { uri: uri.to_string(), ..target_result }),
                sender
            });
        });
    }
}

impl Crawler {
    pub fn new(parent: Addr<SpiderSupervisor>) -> Self {
        let https = HttpsConnector::new();
        let http_client = hyper::Client::builder().build::<_, hyper::Body>(https);
        Self { parent, http_client }
    }
    
    #[allow(dead_code)]
    pub fn parent(&self) -> Addr<SpiderSupervisor> { self.parent.clone() }
}

fn parse_target_body(body: &str) -> Target {
    let scraper = GenWebScraper::new(body);
    scraper.scrape()
    //     if let Some(link) = link.value().attr("href") {
            
    //     }
    //     let href = link.value().attr("href").unwrap();
    // }

}

async fn get_target_body(client: &HyperClient, uri: &Uri) -> DynResult<String> {

    let mut response_result = client.get(uri.clone()).await;
    match response_result.as_mut() {
        Ok(response) => {
            let mut buf = String::new();
        
            while let Some(next) = response.data().await {
                let chunk = next?;
                buf.push_str(&String::from_utf8_lossy(&chunk));
            };
            
            Ok(buf)
        },
        Err(err) => {
            let err = CrawlError::new(uri.to_string(), CrawlErrorInfo::Connection(format!("{:?}", err)));
            Err(Box::new(err))
        },
    }


}

pub fn do_send_crawl(to_crawler: Addr<Crawler>, url: &str) -> bool {
    if is_valid_uri_scheme(url) {
        to_crawler.do_send(Crawl(url.to_string()));
        true
    } else { false }
}