FROM rust:1.79-slim
RUN apt-get update && apt-get install -y libprotobuf-dev protobuf-compiler
WORKDIR /usr/src/app
COPY . .
COPY 1KB.txt /tmp/
COPY 10KB.txt /tmp/
COPY 100KB.txt /tmp/
COPY 1MB.txt /tmp/
COPY 5MB.txt /tmp/
COPY 10MB.txt /tmp/
RUN cargo install --path .
CMD ["grpc-server"]

EXPOSE 50051