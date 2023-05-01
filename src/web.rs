use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    timestamp: std::time::SystemTime,
    uri: String,
    data: Vec<u8>
}

// impl std::fmt::Display for Target {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!()
//     }
// }