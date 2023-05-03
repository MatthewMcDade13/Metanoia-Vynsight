use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Target {
    timestamp: std::time::SystemTime,
    uri: String,
    page_raw: String,
    child_links: Vec<String>,
    parent_link: Option<String>
}

impl Target {
    pub const fn timestamp(&self) -> std::time::SystemTime { self.timestamp }
    pub fn page_raw(&self) -> &str { self.page_raw.as_str() }
    pub fn uri(&self) -> &str { self.uri.as_str() }
    pub fn child_links(&self) -> &[String] { &self.child_links.as_slice() }
    pub fn parent_link(&self) -> Option<&str> { self.parent_link.as_deref() }
}