use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq, Clone)]
pub struct User {
    pub id: u64,
    pub name: String
}