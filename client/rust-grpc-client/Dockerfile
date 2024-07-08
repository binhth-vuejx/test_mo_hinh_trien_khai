FROM rust:1.79-slim
RUN apt-get update && apt-get install -y libprotobuf-dev protobuf-compiler
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .
CMD ["grpc-client"]

EXPOSE 50151