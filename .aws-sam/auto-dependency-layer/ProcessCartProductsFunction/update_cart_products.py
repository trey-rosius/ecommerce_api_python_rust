import json
import os

import boto3
from aws_lambda_powertools import Logger, Tracer
from aws_lambda_powertools.utilities.batch import (
    BatchProcessor,
    EventType,
    process_partial_response,
)
from aws_lambda_powertools.utilities.data_classes.sqs_event import SQSRecord
from aws_lambda_powertools.utilities.typing import LambdaContext

processor = BatchProcessor(event_type=EventType.SQS)
tracer = Tracer()
logger = Logger()

table_name = os.environ.get("TABLE_NAME")
dynamodb = boto3.resource("dynamodb")
table = dynamodb.Table(table_name)


@tracer.capture_method
def record_handler(record: SQSRecord):
    payload: str = record.body
    if payload:
        item: dict = json.loads(payload)
        logger.debug(item)
        logger.debug(f"product id {item['productId']}")
        response = table.update_item(
            Key={
                # "PK": f"USER#{item['user_id']}",
                "PK": f"USER#test@gmail.com",
                "SK": f"PRODUCT#{item['productId']}",
            },
            ConditionExpression="attribute_exists(PK)",
            UpdateExpression="set cart_product_status= :productStatus",
            ExpressionAttributeValues={":productStatus": "ORDERED"},
            ReturnValues="ALL_NEW",
        )

        logger.debug({" update response": response["Attributes"]})


@logger.inject_lambda_context
@tracer.capture_lambda_handler
def lambda_handler(event, context: LambdaContext):
    return process_partial_response(event=event, record_handler=record_handler, processor=processor, context=context)
