use std::marker::PhantomData;

use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Target {
    timestamp: std::time::SystemTime,
    uri: String,
    page_raw: String,
    child_links: Vec<String>,
    parent_link: Option<String>
}

#[allow(dead_code)]
impl Target {

    pub fn new(uri: String, page_raw: String, child_links: Vec<String>, parent_link: Option<String>) -> Self {
        let timestamp = std::time::SystemTime::now();
        Self { timestamp, uri, page_raw, child_links, parent_link }
    }


    pub const fn timestamp(&self) -> std::time::SystemTime { self.timestamp }
    pub fn page_raw(&self) -> &str { self.page_raw.as_str() }
    pub fn uri(&self) -> &str { self.uri.as_str() }
    pub fn child_links(&self) -> &[String] { &self.child_links.as_slice() }
    pub fn parent_link(&self) -> Option<&str> { self.parent_link.as_deref() }
}

// ///
// /// ```
// /// 
// /// UrlDomain(opt(str(sub_domain)), str(domain), str(super_domain))
// /// 
// /// 
// /// ```
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
// pub struct UrlDomain(Option<String>, String, String);

// impl UrlDomain {
//     pub fn sub(&self) -> Option<&str> { self.0.as_deref() }
//     pub fn this(&self) -> &str { &self.1 }
//     pub fn sup(&self) -> &str { &self.2 }
// }

// impl<'a> std::fmt::Display for UrlDomain {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let s: String = self.into();
//         write!(f, "{}", s)
//     }
// }

// impl<'a> From<&UrlDomain> for String {
//     fn from(domain: &UrlDomain) -> Self {
//         domain.into()
//     }
// }

// impl<'a> From<UrlDomain> for String {
//     fn from(url_domain: UrlDomain) -> Self {
//         if let Some(sub) = url_domain.sub() {
//             format!("{}.{}.{}", sub, url_domain.this(), url_domain.sup())
//         } else {
//             format!("{}.{}", url_domain.this(), url_domain.sup())
//         }
//     }
// }


// impl<'a> From<String> for UrlDomain {
//     fn from(domain: String) -> Self {        
//         let parts: Vec<&str> = domain.split('.').collect();
        
//         match parts.len() {
//             3 => {
//                 let sub = Some(parts[0].to_owned());
//                 let this = parts[1].to_owned();
//                 let sup = parts[2].to_owned();
//                 UrlDomain(sub, this, sup)
//             },
//             2 => {
//                 let this = parts[0].to_owned();
//                 let sup = parts[1].to_owned();
//                 UrlDomain(None, this, sup)
//             },
//             1 => UrlDomain(None, parts[0].to_owned(), "undefined".into()),
//             _ => UrlDomain(None, "undefined".into(), "undefined".into())
//         }
//     }
// }

// impl From<&str> for UrlDomain {
//     fn from(domain: &str) -> Self {        
//         Self::from(domain.to_owned())
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
// pub struct Url {
//     full_url: String,
//     scheme: String,
//     domain: UrlDomain,
//     path: String
// }

// impl Url {

//     pub fn new(full_url: &str) -> Self {

//         let full_url: String = full_url.to_owned();

//         let parts: Vec<&str> = full_url
//             .split('/')
//             .filter(|s| !s.is_empty())
//             .collect();

//         let scheme = parts[0]
//             .strip_suffix(':')
//             .unwrap_or(parts[0])
//             .to_owned();

//         let domain = UrlDomain::from(parts[1]);
//         let path = parts[2..].join("/");

//         Self { full_url, scheme, domain, path }
//     }
// }

// #[derive(Debug)]
// pub enum InvalidUrl {
//     NoValidScheme,
//     InvalidDomain,
// }

// impl std::fmt::Display for InvalidUrl {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             InvalidUrl::NoValidScheme => write!(f, "No valid scheme"),
//             InvalidUrl::InvalidDomain => write!(f, "Invalid domain"),
//         }
//     }
// }

// impl std::error::Error for InvalidUrl {}




