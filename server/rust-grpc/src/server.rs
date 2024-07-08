use std::include_str;
use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, Empty};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

const BODY1KB: &str = include_str!("/tmp/1KB.txt");
const BODY10KB: &str = include_str!("/tmp/10KB.txt");
const BODY100KB: &str = include_str!("/tmp/100KB.txt");
const BODY1MB: &str = include_str!("/tmp/1MB.txt");
const BODY5MB: &str = include_str!("/tmp/5MB.txt");
const BODY10MB: &str = include_str!("/tmp/10MB.txt");

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn body1_kb(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = hello_world::HelloReply {
            message: BODY1KB.to_string()
        };
        Ok(Response::new(reply))
    }
    async fn body10_kb(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = hello_world::HelloReply {
            message: BODY10KB.to_string()
        };
        Ok(Response::new(reply))
    }
    async fn body100_kb(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = hello_world::HelloReply {
            message: BODY100KB.to_string()
        };
        Ok(Response::new(reply))
    }
    async fn body1_mb(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = hello_world::HelloReply {
            message: BODY1MB.to_string()
        };
        Ok(Response::new(reply))
    }
    async fn body5_mb(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = hello_world::HelloReply {
            message: BODY5MB.to_string()
        };
        Ok(Response::new(reply))
    }
    async fn body10_mb(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = hello_world::HelloReply {
            message: BODY10MB.to_string()
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}