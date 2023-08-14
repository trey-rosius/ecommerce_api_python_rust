import os
import boto3 as boto3
from aws_lambda_powertools.event_handler import APIGatewayRestResolver
from aws_lambda_powertools.utilities.data_classes.appsync import scalar_types_utils
from aws_lambda_powertools.utilities.typing import LambdaContext
from aws_lambda_powertools.logging import correlation_paths
from aws_lambda_powertools import Logger
from aws_lambda_powertools import Tracer
from aws_lambda_powertools import Metrics
from aws_lambda_powertools.metrics import MetricUnit
from boto3.dynamodb.conditions import Attr, Key
from botocore.exceptions import ClientError


app = APIGatewayRestResolver()
tracer = Tracer()
logger = Logger()

metrics = Metrics(namespace="Ecommerce_api")

table_name = os.environ.get("TABLE_NAME")
dynamodb = boto3.resource("dynamodb")
table = dynamodb.Table(table_name)


# write a function to place order
@app.post("/order")
@tracer.capture_method
def place_order() -> dict:
    # adding custom metrics
    # See: https://awslabs.github.io/aws-lambda-powertools-python/latest/core/metrics/
    metrics.add_metric(name="PlaceOrderInvocations", unit=MetricUnit.Count, value=1)

    if not app.current_event.json_body:
        return {
            "statusCode": 400,
            "body": {"message": "No Request payload"},
        }
    request_payload: dict = app.current_event.json_body
    logger.info(f"request payload is {request_payload}")

    ## Order Status is either ORDERED/CANCELLED/COMPLETED
    ## When you place the order, the status is set to ORDERED
    ## when  you cancel the order, the status is set to CANCELLED
    ## when you complete the order, the status is set to COMPLETED
    order_id = scalar_types_utils.make_id()
    user_id = request_payload["user_id"]
    order_status = "ORDERED"
    order_total = request_payload["order_total"]
    order_items = request_payload["order_items"]

    item = {
        "PK": f"ORDER",
        "SK": f"ORDER#{order_id}",
        "GSI1PK": f"USER#{user_id}",
        "GSI1SK": f"ORDER#{order_id}",
        "user_id": user_id,
        "order_id": order_id,
        "order_status": order_status,
        "order_date": scalar_types_utils.aws_date(),
        "order_total": order_total,
        "order_items": order_items,
    }
    try:
        table.put_item(Item=item)
        return {"statusCode": 200, "body": {"message": "order placed successfully"}}
    except ClientError as err:
        logger.debug(f" failed to place order{err.response['Error']}")
        metrics.add_metric(name="place_order", unit="Count", value=1)
        return {
            "statusCode": 500,
            "body": {"message": f" failed to place order{err.response['Error']}"},
        }


@app.get("/cart/<user_id>")
@tracer.capture_method
def get_cart(user_id: str) -> dict:
    # adding custom metrics
    # See: https://awslabs.github.io/aws-lambda-powertools-python/latest/core/metrics/
    metrics.add_metric(name="GetCartInvocations", unit=MetricUnit.Count, value=1)
    try:
        response = table.query(
            KeyConditionExpression=Key("PK").eq(f"USER#{user_id}")
            & Key("SK").begins_with("PRODUCT#"),
            ProjectionExpression="productId,quantity,cartProductStatus,addedOn,userId",
            FilterExpression=Attr("cart_product_status").eq("PENDING"),
        )
        return {"statusCode": 200, "body": {"cart_items": response["Items"]}}
    except ClientError as err:
        logger.debug(f" failed to get cart{err.response['Error']}")
        metrics.add_metric(name="get_cart_products", unit="Count", value=1)
        return {
            "statusCode": 500,
            "body": {"message": f" failed to get cart{err.response['Error']}"},
        }


@app.post("/cart/{user_id}")
@tracer.capture_method
def add_to_cart(user_id: str) -> dict:
    # adding custom metrics
    # See: https://awslabs.github.io/aws-lambda-powertools-python/latest/core/metrics/
    metrics.add_metric(
        name="AddProductsToCartInvocations", unit=MetricUnit.Count, value=1
    )
    if not app.current_event.json_body:
        return {
            "statusCode": 400,
            "body": {"message": "No Request payload"},
        }
    request_payload: dict = app.current_event.json_body
    logger.info(f"request payload is {request_payload}")

    product_id = request_payload["product_id"]
    quantity = request_payload["quantity"]

    item = {
        "PK": f"USER#{user_id}",
        "SK": f"PRODUCT#{product_id}",
        "userId": user_id,
        "productId": product_id,
        "quantity": quantity,
        "cartProductStatus": "PENDING",
        "addedOn": scalar_types_utils.aws_timestamp(),
    }

    try:
        table.put_item(Item=item)
        return {
            "statusCode": 200,
            "body": {"productId": product_id, "message": "product added to cart"},
        }
    except ClientError as err:
        logger.debug(f" failed to add item to cart{err.response['Error']}")
        metrics.add_metric(name="product_add_to_cart", unit="Count", value=1)
        return {
            "statusCode": 500,
            "body": {"message": f" failed to add item to cart{err.response['Error']}"},
        }


@logger.inject_lambda_context(correlation_id_path=correlation_paths.API_GATEWAY_REST)
@tracer.capture_lambda_handler
# ensures metrics are flushed upon request completion/failure and capturing ColdStart metric
@metrics.log_metrics(capture_cold_start_metric=True)
def lambda_handler(event: dict, context: LambdaContext) -> dict:
    return app.resolve(event, context)
