use serde::{Deserialize,Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub order_status: OrderStatus,
    pub user_id: UserId,
    pub order_id: OrderId,
    pub order_items: OrderItem,
    pub order_total: OrderTotal,
    pub  SK: SK,
    pub order_date: OrderDate,
    pub GSI1SK: GSI1SK,
    pub GSI1PK: GSI1PK,
    pub PK: PK,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderStatus {
    S: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
   pub S: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderId {
    S: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItem {
   pub  L: Vec<OrderItemDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemDetail {
    pub M:OrderItemDetailsMap
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderItemDetailsMap{
    pub productId: ProductId,
     pub userId: UserId,
     pub addedOn: AddedOn,
     pub quantity: Quantity,
     pub cartProductStatus: CartProductStatus,
}

#[derive(Debug, Serialize, Deserialize)]
 pub struct ProductId {
   pub S: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddedOn {
   pub S: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Quantity {
    S: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CartProductStatus {
   pub S: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderTotal {
    N: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SK {
    S: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct OrderDate {
    S: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GSI1SK {
    S: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GSI1PK {
    S: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PK {
    S: String,
}