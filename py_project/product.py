import json
import os

import boto3 as boto3
from aws_lambda_powertools.event_handler import APIGatewayRestResolver
from aws_lambda_powertools.utilities.typing import LambdaContext
from aws_lambda_powertools.logging import correlation_paths
from aws_lambda_powertools import Logger
from aws_lambda_powertools import Tracer
from aws_lambda_powertools import Metrics
from aws_lambda_powertools.metrics import MetricUnit
from boto3.dynamodb.conditions import Key
from botocore.exceptions import ClientError

import ecommerce_api;



app = APIGatewayRestResolver()
tracer = Tracer()
logger = Logger()
metrics = Metrics(namespace="Ecommerce_api")

table_name = os.environ.get("TABLE_NAME")
dynamodb = boto3.resource("dynamodb")
table = dynamodb.Table(table_name)
ddb_client = ecommerce_api.DDBClient()
with open("./product_list.json", "r") as product_list:
    product_list = json.load(product_list)



@app.get("/products")
@tracer.capture_method
def get_products():
    # adding custom metrics
    # See: https://awslabs.github.io/aws-lambda-powertools-python/latest/core/metrics/
    metrics.add_metric(name="GetProductsInvocations", unit=MetricUnit.Count, value=1)

    logger.info(f"Table name {table_name}")

    results = []
    last_evaluated_key = None
    while True:
        if last_evaluated_key:
            response = table.query(
                KeyConditionExpression=Key("PK").eq(f"PRODUCT")
                & Key("SK").begins_with(f"PRODUCT#"),
                ExclusiveStartKey=last_evaluated_key,
            )
        else:
            response = table.query(
                KeyConditionExpression=Key("PK").eq(f"PRODUCT")
                & Key("SK").begins_with(f"PRODUCT#"),
            )

        last_evaluated_key = response.get("LastEvaluatedKey")
        logger.debug(f"response item is {response['Items']}")
        response = list(response["Items"])
        results.extend(response)

        if not last_evaluated_key:
            break
    logger.info(f"fetch_all_products returned {results}")
    return {
        "statusCode": 200,
        "body": {"products": results, "total": len(results)},
    }


@app.get("/products/<product_id>")
@tracer.capture_method
def get_product(product_id: str):
    metrics.add_metric(name="GetProductInvocations", unit=MetricUnit.Count, value=1)
    logger.debug(f"product id {product_id}")
    ddb_client.get_product(table_name,product_id)


    '''
    
    

    try:

        item = table.get_item(Key={"PK": f"PRODUCT", "SK": f"PRODUCT#{product_id}"})

        return {
            "statusCode": 200,
            "body": item["Item"],
        }
    except ClientError as err:
        logger.debug(f"Error while getting product {err.response['Error']}")
        raise err

  '''
@app.post("/loadProducts")
@tracer.capture_method
def load_products():
    # adding custom metrics
    # See: https://awslabs.github.io/aws-lambda-powertools-python/latest/core/metrics/
    metrics.add_metric(name="LoadProductsInvocations", unit=MetricUnit.Count, value=1)

    logger.debug("Retrieving all products: %s", product_list)
    logger.debug(f"item id is {product_list[0]['productId']}")
    with table.batch_writer() as batch:
        for item in product_list:
            batch.put_item(
                Item={
                    "PK": f"PRODUCT",
                    "SK": f"PRODUCT#{item['productId']}",
                    "productId": item["productId"],
                    "category": item["category"],
                    "createdDate": item["createdDate"],
                    "description": item["description"],
                    "modifiedDate": item["modifiedDate"],
                    "name": item["name"],
                    "package": item["package"],
                    "pictures": item["pictures"],
                    "price": item["price"],
                    "tags": item["tags"],
                }
            )

    return {
        "statusCode": 200,
        "body": json.dumps({"message": "uploaded successfully"}),
    }


@logger.inject_lambda_context(correlation_id_path=correlation_paths.API_GATEWAY_REST)
@tracer.capture_lambda_handler
# ensures metrics are flushed upon request completion/failure and capturing ColdStart metric
@metrics.log_metrics(capture_cold_start_metric=True)
def lambda_handler(event: dict, context: LambdaContext) -> dict:
    return app.resolve(event, context)
