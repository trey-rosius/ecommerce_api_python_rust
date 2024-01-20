import json
import os

import boto3
from aws_lambda_powertools.utilities.data_classes.dynamo_db_stream_event import (
    DynamoDBStreamEvent,
    DynamoDBRecordEventName
)
from aws_lambda_powertools import Logger

logger = Logger()
table_name = os.environ.get("TABLE_NAME")
dynamodb = boto3.resource("dynamodb")
table = dynamodb.Table(table_name)

# get sqs queue from environment variable
sqs_queue_url = os.environ.get("UPDATE_CART_PRODUCT_SQS")
sqs = boto3.client("sqs")


# send message to sqs queue
def send_sqs_message(order_items):
    sqs.send_message(
        QueueUrl=sqs_queue_url,
        MessageBody=json.dumps(order_items),
    )
    logger.info(f"Sent message to SQS Queue: {sqs_queue_url}")


def lambda_handler(event, context):
    event: DynamoDBStreamEvent = DynamoDBStreamEvent(event)

    # Multiple records can be delivered in a single event
    for record in event.records:
        if record.event_name == DynamoDBRecordEventName.INSERT:
            logger.info(f"New record detected. Event ID: {record.event_id}")
            logger.info(f"New record detected: {record.dynamodb.new_image}")

            for order_item in record.dynamodb.new_image["order_items"]:
                logger.info(f"Sending message to SQS Queue: {order_item}")

                # send message to sqs queue
                send_sqs_message(order_item)
