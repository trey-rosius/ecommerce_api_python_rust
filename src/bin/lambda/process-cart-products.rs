

use std::env;

use aws_lambda_events::dynamodb::Event;

use rayon::prelude::*;
use lambda_runtime::{LambdaEvent,service_fn, Error};
use aws_config::{BehaviorVersion,load_defaults};
use aws_sdk_sqs::{Client as SqsClient, types::SendMessageBatchRequestEntry};
use tracing::info;
use ecommerce_api::model:: Order;


type E = Box<dyn std::error::Error + Sync + Send + 'static>;
#[tokio::main]
async fn main() -> Result<(), Error> {

    
   // Initialize the AWS SDK for Rust
    let config = load_defaults(BehaviorVersion::v2023_11_09()).await;
      tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();
     let sqs_client = SqsClient::new(&config);
     let queue_name = env::var("UPDATE_CART_PRODUCT_SQS").expect("Couldn't get the queue url");
     let sqs_client_ref = &sqs_client;
    let queue_name_ref = &queue_name;
  lambda_runtime::run(service_fn(|request:  LambdaEvent<Event>| {
        process_dynamodb_streams( request,sqs_client_ref,queue_name_ref)
    }))
    .await?;

    Ok(())
}


// update post 
async fn process_dynamodb_streams(
    event: LambdaEvent<Event>,
     sqs_client: &SqsClient,
      sqs_queue_url: &String,
) -> Result<(), E> {

    let events:Vec<String> = event.payload.records
    .par_iter().map(|record| {
      
  let new_image =
                        serde_json::to_string(&record.clone().change.new_image).unwrap();

       
                  
                        info!("new image record {:?}",&record.clone().change.new_image);
                        info!("new image String is {}",new_image);

                       

                        new_image
    }).collect();
    info!("all images String is {:?}",events);
    for item in events{ 
          let order: Order = serde_json::from_str(&item).unwrap();
          info!("structured order is {:?}",order);
         sqs_client.send_message()
                        .queue_url(sqs_queue_url)
                        .message_body(item)
                        .send()
                        .await?;
    }
   

  

 
    Ok(())
   



}

