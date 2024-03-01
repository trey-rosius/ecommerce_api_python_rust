# Optimizing Application Performance with Rust.
# Building Performant EDA Applications with Python and Rust

## Project Recap(add more content)
In a previous workshop, [Building event driven applications with AWS sam and python](https://www.educloud.academy/content/eb8782d3-4281-484e-9f4b-720857db74c0/)
we built an ecommerce ordering service using AWS SAM, API Gateway, Lambda and Python.

In that workshop, whenever a new order is added to the dynamodb table, a lambda function gets triggered. This function grabs the order and pushed it as a message into an SQS Queue for processing.
Another lambda function polls that SQS queue and updates the order record in the DynamoDB table.

In this workshop, we'll be rewriting both lambda functions using Rust.

Why ?

Because Rust is extremely fast,memory and resource efficient and thread safe. Also for the fun of it.

Checkout this article to get a complete understanding of why Rust ? [“Rustifying” Serverless: Boost AWS Lambda performance with Rust](https://community.aws/content/2ZSJAWJt2VbYrGOXPTv1rfmmLSP/rustifying-serverless-boost-aws-lambda-performance-with-rust?lang=en)

## Prerequisites
To complete this workshop, you'll need
- An AWS Account
- A Python installation
- A [rust installation](https://www.rust-lang.org/tools/install)
- An understanding of how to build Serverless APIs with AWS .

## Clone the project
Please clone this repository , it has all configurations and the code

https://github.com/trey-rosius/ecommerce_api_python_rust

## Project Structure
![alt text](https://raw.githubusercontent.com/trey-rosius/ecommerce_api_python_rust/master/assets/python.png)

Highlighted on the screenshot are the 2 lambda functions we'll be rewriting in rust. Let's go ahead to
build and deploy the application, in order to make sure everything works well, before we start refactoring.

## Deploy Project
From the root of the project folder, navigate to the `src` folder and install all dependencies in the requirements.txt file
by running

`pip install -r requirements.txt`.

Once installed successfully, navigate out of the `src` folder and run the following commands to build and deploy
the application.

```bash
sam build
sam deploy --guided
```

Once deployed, you'll have to create an API KEY with a Usage plan in API gateway. Then use the API KEY
and API gateway endpoint generated as output from the deployment step to test the endpoints.

You can use postman or any other api testing software of your choice.

![alt text](https://raw.githubusercontent.com/trey-rosius/ecommerce_api_python_rust/master/assets/b.png)


![alt text](https://raw.githubusercontent.com/trey-rosius/ecommerce_api_python_rust/master/assets/auth.png)


## Adding Rust to the project
We'll use `cargo`, Rust's package manager to initialize for rust in our project.

From the root folder, run the command

`cargo init`

This command initializes a rust project inside an already created folder. A couple of files have been added to the project.

- Cargo.toml file that'll contain all the crates for this project
- src folder(which already existed)
- main.rs file which we won't use.

Add the following to your `samconfig.toml` file:

```yaml
[default.build.parameters]
beta_features = true
[default.sync.parameters]
beta_features = true
```

## Add Crates to Cargo.toml

In Rust, dependencies are known as crates.

Rust has conditional compilation flags that you can use to trigger different features in crates.

For example, for the crate below, we need strongly typed lambda events for dynamodb and sqs only.

Using conditional flags, we can ensure that we import only those events.

`aws_lambda_events = { version = "0.8.3",optional = true,features = ["dynamodb","sqs"]}`

Inside `Cargo.toml`, under `dependencies`, add the following crates

```toml
aws-config = "1.1.1"
aws-sdk-dynamodb = "1.9.0"
tokio = { version = "1.21.1", features = ["full"] }

lambda_runtime = { version = "0.8.3", optional = true}
rayon = { version = "1.8.1", optional = true }
serde = "1.0.136"
serde_dynamo = "4.2.7"
aws_lambda_events = { version = "0.8.3",optional = true,features = ["dynamodb","sqs"]}

serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.2", features = ["fmt", "json"] }
aws-sdk-sqs = {version="1.9.0",optional=true}
```

- aws-config provides implementations of region and credential resolution.
- aws-sdk-dynamodb provides dynamodb implementations.
- tokio gives rust async superpowers.
- serde provides serialization/deserialization of rust datastructures
- serde_dynamo provides a way to serialize and deserialize between data stored in these items and strongly-typed Rust data structures.
- lambda_runtime: This package makes it easy to run AWS Lambda Functions written in Rust.
- tracing is a framework for instrumenting Rust programs to collect structured, event-based diagnostic information.

## Process Cart Products
This lambda function listens to dynamodb stream Order events with order status as `ORDERED`, extracts the products
from the order and pushes them into an SQS Queue.

Let's get started.

Firstly, we have to create an event source mapping with the Dynamodb stream as event source and the `ProcessCartProductFunction`
as the lambda function to be invoked.

This lambda function only gets triggered when this pattern matches `'{ "dynamodb": { "NewImage": { "order_status": { "S": ["ORDERED"] } } } }'`

That is, when the `order_status` for a newly added item equals `ORDERED`

### Event Source Mapping Resource
```yaml

  EventSourceDDBTableStream:
    Type: AWS::Lambda::EventSourceMapping
    Properties:
      BatchSize: 1
      Enabled: True
      FilterCriteria:
        Filters:
          - Pattern: '{ "dynamodb": { "NewImage": { "order_status": { "S": ["ORDERED"] } } } }'
      EventSourceArn: !GetAtt EcommerceAppTable.StreamArn
      FunctionName: !GetAtt ProcessCartProductsFunction.Arn
      StartingPosition: LATEST


```

### Lambda Function Resource
Then for the lambda function resource,

```yaml
  ProcessCartProductsFunction:
    Type: AWS::Serverless::Function # More info about Function Resource: https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-resource-function.html
    Metadata:
      BuildMethod: rust-cargolambda
      BuildProperties:
        Binary: process-cart-products
    Properties:
      CodeUri: ./
      Handler: bootstrap
      Runtime: provided.al2
      Description: Lambda function listens to dynamodb streams, gets order and sends order to sqs
      Architectures:
        - arm64
      Policies:
        - DynamoDBStreamReadPolicy:
            TableName: !Ref EcommerceAppTable
            StreamName:
              !Select [3, !Split ["/", !GetAtt EcommerceAppTable.StreamArn]]

    Connectors:
      SQSConnectors:
        Properties:
          Destination:
            Id: UpdateCartProductsSQS
          Permissions:
            - Write
```
#### What's happening above ?
In order to deploy rust code on lambda, AWS provides 2 runtimes `provided.al2023 or provided.al2`.
In this project, we're using the `provided.al2` runtime.

Because our template would contain more than one rust lambda function, we've added a binary parameter,
pointing to the binary of this lambda function defined in Cargo.yaml(Which we'll be looking at in a moment).

`Binary: process-cart-products`

Handler is `bootstrap`.

`codeUri` specifies where our rust code is located and build method is `rust-cargoLambda`.

To learn more about custom runtimes, see [Custom AWS Lambda runtimes](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-custom.html) in the AWS Lambda Developer Guide.

We've also added permissions for the lambda functions to read from a dynamodb stream and write to an SQS queue.

## Creating Process Cart Product Lambda Function
Inside the `src` folder, create a `bin` folder and then create a `lambda` folder inside the `bin` folder.

Create a file called `process-cart-products.rs` inside the lambda folder.

So the folder structure now looks like so

```bash
  src
    |-bin
      |-lambda
        |- process-cart-product.rs

```

Open up the Cargo.yaml file and add the `path` and `name` to the Rust binary executable.


```toml
[[bin]]
name = "process-cart-products"
path ="src/bin/lambda/process-cart-products.rs"
test= false

```
Remember that the name `process-cart-products` was the same name we specified in the `template.yaml` file for `binary`.
A rust project can have multiple binaries, represented with these `[[bin]]` in a cargo.toml file.

We'll focus mainly on the juicy parts of the lambda function.

This function asynchronously listens to records from the dynamodb stream event payload(`LambdaEvent<Event>`), loops through the records and sends SQS messages
to an SQS queue.

We use the `serde_json` crate to convert the dynamodb item into a string, suitable for adding to the
message body of the sqs message.

```rust
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
```

## Create Update Cart Status Resources and Function
This function processes messages from the SQS queue and updates the cart product status in the dynamodb table.

### Lambda function resource

```yaml
  UpdateCartProductsDLQ:
    Type: AWS::SQS::Queue
  UpdateCartProductsFunction:
    Type: AWS::Serverless::Function # More info about Function Resource: https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/sam-resource-function.html
    Metadata:
      BuildMethod: rust-cargolambda
      BuildProperties:
        Binary: update-cart-status
    Properties:
      CodeUri: ./
      Handler: bootstrap
      Runtime: provided.al2
      Architectures:
        - arm64
      Description: Lambda function pulls the sqs queue messages and processes the order items
      Events:
        RetrieveFromSQS:
          Type: SQS
          Properties:
            Queue: !GetAtt UpdateCartProductsSQS.Arn
            BatchSize: 5
            FunctionResponseTypes:
              - ReportBatchItemFailures
```
This lambda function is set up to receive messages from SQS in batches of 5 and has same properties as the
function we defined above.

Create a file named `update-cart-status.rs` inside of `src/bin/lambda`.

Inside `Cargo.yaml` add the path and name to the binary executable.
```toml

[[bin]]
name = "update-cart-status"
path ="src/bin/lambda/update-cart-status.rs"
test = false
```

Asynchronously receive SQS Events and loop through them .
```rust
async fn function_handler(
    event: LambdaEvent<SqsEventObj<EventRecord>>,
    table_name: &String,
    client: &Client,
) -> Result<(), Error> {
    info!("sqs payload {:?}", event.payload);
    info!("sqs payload records {:?}", event.payload.records[0]);

    for event_record in event.payload.records {
```

Using the `serde_dynamodb` crate, map the item from SQS to a custom Struct. In our case, we created a struct
called `Order` which represents our Order Item and used the `serde` library to serialize and deserialize data.

#### Order Item Struct
```rust

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
    #[serde(rename = "GSI1SK")]
    gsi1sk: GSI1SK,
    #[serde(rename = "GSI1PK")]
    gsi1pk: GSI1PK,
    #[serde(rename = "PK")]
    pk: PK,
}
```

```rust
      let new_image = event_record.body.change.new_image.into_inner();

       let record_data: Order = serde_dynamo::from_item(new_image).unwrap();
```
Messages in SQS are received in batches of 5. So we loop over them and update each record in the dynamodb table.

`.update_expression("SET cartProductStatus = :cartProductStatus")`

```rust

  // update item in the DynamoDB table
            let res = client
                .update_item()
                .table_name(table_name)
                .set_key(Some(key_map))
                .condition_expression("attribute_exists(PK)")
                .update_expression("SET cartProductStatus = :cartProductStatus")
                .expression_attribute_values(
                    ":cartProductStatus",
                    AttributeValue::S("ORDERED".into()),
                )
                .return_values(ReturnValue::UpdatedNew)
                .send()
                .await;
```
