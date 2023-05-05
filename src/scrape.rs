use std::cell::{RefCell, Cell};

use scraper::Html;

use crate::web::Target;



pub trait WebScraper {
    fn scrape(&self) -> Target;
}

pub struct ScrapeReducer<T>(pub RefCell<T>) where T: Clone;

impl<T> ScrapeReducer<T> where T: Clone {
    
    pub fn extract<Extr>(&self, html: &Html, extr: &Extr) -> &Self where Extr: WebExtractor<T> {
        let acc = self.0.borrow().clone();
        let res = extr.extract_into(&acc, html);
        self.0.replace(res);
        self
    }

    pub fn result(&self) -> T {
        self.0.borrow().clone()
    }
}

pub trait WebExtractor<T> {
    fn extract_into(&self, acc: &T, html: &Html) -> T;
}

