use std::env;

use aws_lambda_events::dynamodb::Event;

use aws_config::{load_defaults, BehaviorVersion};
use aws_sdk_sqs::Client as SqsClient;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use rayon::prelude::*;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize the AWS SDK for Rust
    let config = load_defaults(BehaviorVersion::v2023_11_09()).await;
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::INFO)
        .with_current_span(false)
        .without_time()
        .with_target(false)
        .init();
    let sqs_client = SqsClient::new(&config);
    let queue_name = env::var("UPDATE_CART_PRODUCT_SQS").expect("Couldn't get the queue url");

    lambda_runtime::run(service_fn(|request: LambdaEvent<Event>| {
        process_dynamodb_streams(request, &sqs_client, &queue_name)
    }))
    .await?;

    Ok(())
}

// update post
async fn process_dynamodb_streams(
    event: LambdaEvent<Event>,
    sqs_client: &SqsClient,
    sqs_queue_url: &String,
) -> Result<(), Error> {
    info!("(BatchSize)={:?}", event.payload.records.len());
    for record in &event.payload.records {
        let new_image = serde_json::to_string(&record.change.new_image).unwrap();
        sqs_client
            .send_message()
            .queue_url(sqs_queue_url)
            .message_body(new_image)
            .send()
            .await?;
    }

    Ok(())
}
