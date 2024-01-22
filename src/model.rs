use std::{collections::HashMap};

use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize,Serialize};
use aws_smithy_types::error::operation::BuildError;
use crate::error::Error;

// By default, struct field names are deserialized based on the position of
// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize,Serialize)]
pub struct Record {
    pub order_id:String,
    pub order_total:i64,
    pub order_status:String,
    pub user_id:String,
    pub order_date:String,
    #[serde(rename = "GSI1PK")]
    pub gsi1pk:String,
     #[serde(rename = "GSI1SK")]
    pub gsi1sk:String,
     #[serde(rename = "PK")]
    pub pk:String,
     #[serde(rename = "SK")]
    pub sk:String,
     #[serde(default = "Vec::new")]
    pub order_items:Vec<RecordItem>

  
}
#[derive(Debug, Deserialize,Serialize)]
pub struct RecordItem{
     #[serde(rename = "cartProductStatus")]
    pub cart_product_status:String,
    #[serde(rename = "userId")]
    pub user_id:String,
    pub quantity:i32,
    #[serde(rename = "productId")]
    pub product_id:String,
    #[serde(rename = "addedOn")]
    pub added_on:String,
    

}
fn as_string(val: Option<&AttributeValue>, default: &String) -> String {
    if let Some(v) = val {
        if let Ok(s) = v.as_s() {
            return s.to_owned();
        }
    }
    default.to_owned()
}
/* 
fn as_objectvec(val: Option<&AttributeValue>) -> Vec<OrderItem> {
    if let Some(val) = val {
        if let Ok(val) = val.as_l() {
            if let Ok(val) =val.as_m{
 return val
                .iter()
                .map(|v| as_string(Some(v), &"".to_string()))
                .collect();
            }
           
        }
    }
    // val
    //         .map(|v| v.as_l())
    //         .unwrap_or_else(|| Ok(&Vec::<AttributeValue>::new()))
    //         .unwrap_or_else(|_| &Vec::<AttributeValue>::new())
    //         .iter()
    //         .map(|v| as_string(Some(v), &"".to_string()))
    //         .collect();
    vec![]
}
*/
fn as_i32(val: Option<&AttributeValue>, default: i32) -> i32 {
    if let Some(v) = val {
        if let Ok(n) = v.as_n() {
            if let Ok(n) = n.parse::<i32>() {
                return n;
            }
        }
    }
    default
}

/* 
impl TryFrom<&HashMap<String, AttributeValue>>  for OrderItem{
    type Error = BuildError;

    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
       Ok(OrderItem{
        cart_product_status: as_string(value.get("cart_product_status"), &"".to_string()),
        user_id: as_string(value.get("user_id"), &"".to_string()),
        quantity: as_i32(value.get("quantity"), 0),
        product_id: as_string(value.get("product_id"), &"".to_string()),
        added_on: as_string(value.get("added_on"), &"".to_string()),
    })
         
    }
}

impl TryFrom<&HashMap<String, AttributeValue>> for  Order {
    type Error = Error;

    /// Try to convert a DynamoDB item into a  Order
    ///
    /// This could fail as the DynamoDB item might be missing some fields.
    fn try_from(value: &HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        Ok(Order {
           
            order_id:as_string(value.get("order_id"), &"".to_string()),
           
            order_total: as_string(value.get("order_total"), &"".to_string()),
            order_status: todo!(),
            user_id: todo!(),
            order_date: todo!(),
            gsi1pk: todo!(),
            gsi1sk: todo!(),
            pk: todo!(),
            sk: todo!(),
            order_items: vec![
            OrderItem{  cart_product_status: as_string(value.get("cart_product_status"), &"".to_string()),
        user_id: as_string(value.get("user_id"), &"".to_string()),
        quantity: as_i32(value.get("quantity"), 0),
        product_id: as_string(value.get("product_id"), &"".to_string()),
        added_on: as_string(value.get("added_on"), &"".to_string()), }
            ]
        })
    }
}

*/

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