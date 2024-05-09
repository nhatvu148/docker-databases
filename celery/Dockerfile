FROM python:3.10-slim

WORKDIR /app
COPY requirements.txt /app/
RUN pip install -r requirements.txt
COPY . /app/

CMD ["celery", "-A", "tasks", "worker", "--loglevel=info"]
# "--concurrency=2"
