use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemModel {
    // #[serde(skip_deserializing, default)]
    pub id: Option<i32>,
    pub ProductId: i32,
    pub Name: String,
    pub Quantity: i32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetItemModel{
    pub ProductId: i32,
    pub Name: String,
    pub Quantity: i32,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateItemModel{
    pub Name: String,
    pub Quantity: i32,
}