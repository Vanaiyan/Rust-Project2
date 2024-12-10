
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProductModel {
    pub uuid: Uuid,
    pub Name: String,
    pub Description: String,
    pub Created_at: NaiveDateTime
}

#[derive(Serialize,Deserialize)]
pub struct CreateProductModel {
    pub Name: String,
    pub Description: String,
}