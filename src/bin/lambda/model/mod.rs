use serde::{Deserialize,Serialize};
 
#[derive(Debug, Serialize, Deserialize)]

pub struct Order {
    pub order_status: OrderStatus,
    pub user_id: UserId,
    pub order_id: OrderId,
    pub order_items: OrderItem,
    pub order_total: OrderTotal,
    #[serde(rename = "SK")]
     sk: SK,
     order_date: OrderDate,
    #[serde(rename= "GSI1SK")]
     gsi1sk: GSI1SK,
    #[serde(rename = "GSI1PK")]
    gsi1pk: GSI1PK,
    #[serde(rename = "PK")]
    pk: PK,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct OrderStatus {
    s: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct UserId {
   pub s: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct OrderId {
    s: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct OrderItem {
   pub  l: Vec<OrderItemDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct OrderItemDetail {
    pub m:OrderItemDetailsMap
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderItemDetailsMap{
  
    pub product_id: ProductId,
 
     pub user_id: UserId,

     pub added_on: AddedOn,

    quantity: Quantity,

     pub cart_product_status: CartProductStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
 pub struct ProductId {
   pub s: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct AddedOn {
   pub s: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct Quantity {
    s: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct CartProductStatus {
   pub s: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct OrderTotal {
    n: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct SK {
    s: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct OrderDate {
    s: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct GSI1SK {
    s: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct GSI1PK {
    s: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct PK {
    s: String,
}