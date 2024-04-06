use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MyData {
    pub id: u32,
    pub name: String,
}
