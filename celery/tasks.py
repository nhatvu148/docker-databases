from celery import Celery
from time import sleep

# app = Celery('app_queues', broker='amqp://guest:guest@localhost:5672//', backend='db+sqlite:///db.sqlite3')
app = Celery('app_queues', broker='amqp://guest:guest@rabbitmq:5672//')

@app.task
def reverse(text):
    sleep(5)
    return text[::-1]

@app.task
def add(x, y):
    return x + y