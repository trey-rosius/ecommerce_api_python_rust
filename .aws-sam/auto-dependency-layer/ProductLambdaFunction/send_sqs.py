# write a function to send a batch of messages to sqs
import boto3
import json
import random
import time
import uuid

sqs = boto3.resource('sqs')
queue = sqs.get_queue_by_name(QueueName='test_queue')
# queue = sqs.create_queue(QueueName='test_queue')

def send_message(message):
    response = queue.send_message(MessageBody=message)
    return response

def send_messages(messages):
    for message in messages:
        send_message(message)

def send_messages_batch(messages):
    response = queue.send_messages(Entries=messages)
    return response

