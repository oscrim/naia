FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y openssl

COPY server-demo server


CMD ["sh", "-c", "./server"]