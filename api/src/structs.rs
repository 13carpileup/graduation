use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String
}