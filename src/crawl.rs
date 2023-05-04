
use actix::prelude::*;
use futures_util::TryFutureExt;
use hyper::{http::response, Uri};
use hyper::{body::HttpBody as _, Client};
use hyper_tls::HttpsConnector;

use crate::{spider_sup::SpiderSupervisor, common::{HyperClient, DoneCrawl, DynResult}, web::Target};



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

            let body = get_target_body(&client, &uri).await.unwrap();
            let child_links = parse_target_body(&body);

            parent.do_send(DoneCrawl {
                result: Ok(Target::new(uri.to_string(), body, child_links, None)),
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

fn parse_target_body(body: &str) -> Vec<String> {
    todo!()
}

async fn get_target_body(client: &HyperClient, uri: &Uri) -> DynResult<String> {
    /*
        let client = Client::new()?;
        let uri = Uri::from_static("https://stackoverflow.com/questions/16902869/best-way-to-parse-an-int-in-javascript");
        let response = client.get(uri).await?;
        let body = response.text().await?;
        let document = Html::parse_document(&body);
        let selector = Selector::parse("a").unwrap();
        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                println!("{}", href);
            }
        }
        Ok(())
     */

    let mut response = client.get(uri.clone()).await?;
    let mut buf = String::new();

    while let Some(next) = response.data().await {
        let chunk = next?;
        buf.push_str(&String::from_utf8_lossy(&chunk));
    };

    Ok(buf)
}
