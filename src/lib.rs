use pyo3::{prelude::*, types::PyDict, exceptions::PyValueError};
use lambda_runtime::{Error};

use serde_dynamo::aws_sdk_dynamodb_1::from_item;
use aws_config::{BehaviorVersion,load_defaults};
use serde::{Deserialize, Serialize};
use tracing::{info,error};
use std::{sync::Arc, env, collections::HashMap};
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use tokio::runtime::Runtime;

#[pyclass]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
struct ProductItem{
    category:String,
    createdDate:String,
    modifiedDate:String,
    name:String,
    package:Vec<(String,i32)>,
    pictures:Vec<String>,
    price:i128,
    productId:String,
    tags:Vec<String>,
    
}

#[pyclass]
struct DDBClient{
 client:Client,
}

#[pymethods]
impl DDBClient{
   #[new]
    fn new(kwargs: Option<&PyDict>) -> PyResult<Self> {
        let rt = Runtime::new().unwrap();
        let client = rt.block_on(async {
           tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();
            let config = load_defaults(BehaviorVersion::v2023_11_09()).await;
            DDBClient {
                client: Client::new(&config),
                
            }
        });

        Ok(client)
    }

fn get_product(&self,table_name:String,product_id:String)->PyResult<ProductItem>{

 let rt = Runtime::new().unwrap();
        let result = rt.block_on(async {
            let result = get_product_internal(&self.client,table_name,product_id).await;
            result.map_err(|err| PyValueError::new_err(err.to_string()))
        });
        result
    }

}
    


async fn get_product_internal(client: &Client,table_name:String,product_id:String) -> Result<ProductItem, Error> {
     // Building a Composite Key
    let key_map: HashMap<String, AttributeValue> = [
        ("PK".into(), AttributeValue::S("PRODUCT".into())),
        ("SK".into(), AttributeValue::S(format!("PRODUCT#{product_id}")))
    ]
    .iter()
    .cloned()
    .collect();


    tracing::info!("product item id is {:?}",key_map);
 // Get the item in the DynamoDB table
    match client
        .get_item()
        .table_name(table_name)
        .set_key(Some(key_map))
        
        .send()
        .await{
              Ok(result) => {
            // leveraging serde_dynamo

            let i: ProductItem = from_item(result.item.unwrap())?;

            tracing::info!("Product item is {:?}",i);
            Ok(i)
        }
        Err(e) => Err(e.into()),
            
        }

     
        
}



/// A Python module implemented in Rust.
#[pymodule]
fn ecommerce_api(_py: Python, m: &PyModule) -> PyResult<()> {
      m.add_class::<DDBClient>()?;
    Ok(())
}

