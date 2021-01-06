use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "Id")]
    pub id: uuid::Uuid,
    #[serde(rename = "Username")]
    pub username: String,
}
