use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Customer {
    #[serde(default)]
    pub id: i64,
    pub full_name: String,
    pub email: String,
    pub phone: String,
    #[serde(default)]
    pub created_at: String,
}
