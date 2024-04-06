use serde::{Serialize, Deserialize};

#[derive(Hash, Serialize, Deserialize, Clone)]
pub struct Message {
    pub content: String,
    pub timestamp: u128,
    pub from: u8 // 0 me 1 other
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content && self.timestamp == other.timestamp
    }
}

impl Eq for Message {}