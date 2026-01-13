use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    #[serde(default)]
    pub id: i64,
    pub customer_id: i64,
    pub account_number: String,
    pub balance: f64,
    pub account_type: String,
    #[serde(default)]
    pub created_at: String,
}