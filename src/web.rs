use std::cell::{Cell, RefCell};

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::scrape::{WebScraper, ScrapeReducer, WebExtractor};


#[derive(Debug, Clone)]
pub struct GenWebScraper {
    body: String,
    document: Html,
}

impl WebScraper for GenWebScraper {
    fn scrape(&self) -> Target {

        let target = {
            let t = Target::default();
            ScrapeReducer(RefCell::new(t))
        };
        target
            .extract(&self.document, &LinkExtractor)
            .extract(&self.document, &RawExtractor)
            .result()
    }
}

impl GenWebScraper {

    pub fn new(body: &str) -> Self {
        Self {
            body: body.to_owned(),
            document: Html::parse_document(&body),
        }
    }

    pub fn body(&self) -> &str { &self.body }
}

struct LinkExtractor;
impl WebExtractor<Target> for LinkExtractor {
    fn extract_into(&self, acc: &Target, html: &Html) -> Target {
        let mut child_links = acc.child_links.clone();
        let anchor = Selector::parse("a").unwrap();
        for element in html.select(&anchor) {
            if let Some(link) = element.value().attr("href") {
                child_links.push(link.to_owned());
            }
        }

        Target {
            child_links,
            ..acc.clone()
        }
    }
}

struct RawExtractor;
impl WebExtractor<Target> for RawExtractor {
    fn extract_into(&self, acc: &Target, doc: &Html) -> Target {
        Target {
            page_raw: doc.html(),
            ..acc.clone()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Target {
    pub timestamp: std::time::SystemTime,
    pub uri: String,
    pub page_raw: String,
    pub child_links: Vec<String>,
    pub parent_link: Option<String>
}

impl Default for Target {
    fn default() -> Self {
        Self {
            timestamp: std::time::SystemTime::now(),
            uri: String::new(),
            page_raw: String::new(),
            child_links: Vec::new(),
            parent_link: None
        }
    }
}
// TODO :: Verify all these scheme on the match work for hyper::Uri
pub fn is_valid_uri_scheme(uri: &str) -> bool {
    if let Some(parts) = uri.split_once("//") {
        let scheme = parts.0;
        match scheme {
            "http:" | "https:"
            | "file:"
            | "ftp:" | "sftp:"
            | "ws:" => true,
            _ => false
        }
    } else { false }

}