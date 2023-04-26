use std::sync::{Arc, RwLock};

use actix::{Addr, Message};

pub struct RwLockBucket<T: Clone>(Arc<RwLock<Vec<T>>>);


/// ```
/// struct RwLockBucket<T>(Arc<RwLock<Vec<T>>>)
/// ```
pub type ActorBucket<T> = RwLockBucket<Addr<T>>; 

impl<T: Clone> RwLockBucket<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self(Arc::new(RwLock::new(data)))
    }
    
    pub fn with_capacity(size: usize) -> Self {
        let v = Vec::with_capacity(size);
        Self::new(v)
    }

    pub fn len(&self) -> usize {
        if let Ok(data) = self.0.read() {
            data.len()
        } else { 0 }
    }

    pub fn write_at(&mut self, val: T, index: usize) {
        if let Ok(mut data) = self.0.as_ref().write() {            
            if index < data.len() { 
                data.insert(index, val) 
            } 
        }
    }

    pub fn write_pop(&mut self) -> Option<T> {
        if let Ok(mut data) = self.0.as_ref().write() {
            data.pop()
        } else { None }
    }

    pub fn write_push(&mut self, val: T) {
        if let Ok(mut data) = self.0.as_ref().write() {
            data.push(val);
        }
    }

    pub fn read(&self, index: usize) -> Option<T> {
        if let Ok(data) = self.0.read() {
            if let Some(x) = data.get(index) {
                Some(x.clone())
            } else { None }
        } else { None }
    }
 }

impl<T: Clone> Default for RwLockBucket<T> {
    fn default() -> Self {
        Self(Arc::new(RwLock::new(Vec::new())))
    }
}


#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct Kill;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct PopTarget;

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct PushTarget(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct DoneCrawl(pub String);


#[derive(Message)]
#[rtype(result = "()")]
pub struct SpawnCrawler;

type TargetUri = String;
#[derive(Message)]
#[rtype(result = "()")]
pub struct Crawl(pub TargetUri);