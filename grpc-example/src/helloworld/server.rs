use std::net::SocketAddr;
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use crate::hello_world::greeter_server::{Greeter, GreeterServer};


pub mod hello_world {
    tonic::include_proto!("grpc.example.helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<hello_world::HelloRequest>) -> Result<Response<hello_world::HelloReply>, Status> {
        println!("获取到了请求 : {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("你好, {}!", request.into_inner().name)
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "127.0.0.1:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    println!("GreeterServer 监听: {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;


    Ok(())
}