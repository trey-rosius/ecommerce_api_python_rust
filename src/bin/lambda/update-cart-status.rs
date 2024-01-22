


use std::{env, collections::HashMap};

use aws_lambda_events::event::sqs::SqsEventObj;
use aws_sdk_dynamodb::{Client, types::{AttributeValue, ReturnValue}};
use aws_config::{BehaviorVersion,load_defaults};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use ecommerce_api::model::Order;
use tracing::{info,error};


/// Main function
#[tokio::main]
async fn main() -> Result<(), Error> {

      tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    // Initialize the AWS SDK for Rust
    let config = load_defaults(BehaviorVersion::v2023_11_09()).await;

      let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let dynamodb_client = Client::new(&config);
     let table_name_ref = &table_name;
    let dynamodb_client_ref = &dynamodb_client;

   

        let func = service_fn(move |event| function_handler(event,table_name_ref, dynamodb_client_ref) );

    run(func).await?;
    Ok(())
  
}

async fn function_handler(
    event: LambdaEvent<SqsEventObj<serde_json::Value>>,
   table_name:&String,
    client: &Client,
) -> Result<(), Error> {
  info!("sqs payload {:?}",&event.payload);
  info!("sqs payload records {:?}",&event.payload.records[0]);

 
  for data_more in &event.payload.records{

    let record_data:Order = serde_json::from_value(data_more.body.clone()).unwrap();
     info!("Data retrieved from sqs {:?}",record_data);
   
     
     for item in record_data.order_items.L{

         let product_id = &item.M.productId.S;
        let user_id = &item.M.userId.S;
let key_map: HashMap<String, AttributeValue> = [
   

        ("PK".into(), AttributeValue::S(format!("USER#{user_id}"))),
        ("SK".into(), AttributeValue::S(format!("PRODUCT#{product_id}")))
         ]
    .iter()
    .cloned()
    .collect();

      // update item in the DynamoDB table
    let res =  client
        .update_item()
        .table_name(table_name)
         .set_key(Some(key_map))
        .condition_expression("attribute_exists(PK)")
        .update_expression("SET cartProductStatus = :cartProductStatus")
        .expression_attribute_values(":cartProductStatus",AttributeValue::S("ORDERED".into()) )
        .return_values(ReturnValue::UpdatedNew)
       
        .send()
        .await;

     match res {
        Ok(output) =>{
            info!("Item updated successfully {:?}",output)
        },
        Err(err) => {
            error!("An error occured while updating item {:?}",err);
            return Err(Box::new(err))
          
        },
    };
     }
    

      
   


  }

 
    
    Ok(())
       
    }