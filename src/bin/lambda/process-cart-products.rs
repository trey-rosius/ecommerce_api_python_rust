use std::env;

use aws_lambda_events::dynamodb::Event;

use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_sqs::Client as SqsClient;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use rayon::prelude::*;
use tracing::info;

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
    lambda_runtime::run(service_fn(|request: LambdaEvent<Event>| {
        process_dynamodb_streams(request, sqs_client_ref, queue_name_ref)
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
    let events: Vec<String> = event
        .payload
        .records
        .par_iter()
        .map(|record| {
            let new_image = serde_json::to_string(&record.clone().change.new_image).unwrap();

            info!("new image record {:?}", &record.clone().change.new_image);

            new_image
        })
        .collect();
    info!("all images String is {:?}", events);
    for item in events {
        sqs_client
            .send_message()
            .queue_url(sqs_queue_url)
            .message_body(item)
            .send()
            .await?;
    }

    Ok(())
}
