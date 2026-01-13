use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    #[serde(default)]
    pub id: i64,
    pub account_id: i64,
    pub amount: f64,
    pub transaction_type: String,
    pub description: Option<String>,
    #[serde(default)]
    pub created_at: String,
}
