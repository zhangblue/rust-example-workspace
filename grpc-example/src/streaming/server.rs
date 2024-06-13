use std::net::SocketAddr;
use tonic::{Request, Response, Status, Streaming};
use tonic::codegen::tokio_stream::StreamExt;
use tonic::transport::Server;
use crate::streaming_example::{MsgRequest, MsgResponse};

pub mod streaming_example {
    tonic::include_proto!("grpc.example.streaming");
}

#[derive(Default)]
pub struct MsgReceiver {}

#[tonic::async_trait]
impl streaming_example::msg_api_server::MsgApi for MsgReceiver {
    async fn server_streaming_echo(&self, request: Request<Streaming<MsgRequest>>) -> Result<Response<MsgResponse>, Status> {
        let mut stream = request.into_inner();
        while let Some(Ok(msg)) = stream.next().await {
            println!("current msg = [{:?}]", msg);
        }
        Ok(Response::new(MsgResponse::default()))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let add: SocketAddr = "127.0.0.1:50051".parse()?;
    let msg_receiver = MsgReceiver::default();
    Server::builder()
        .add_service(streaming_example::msg_api_server::MsgApiServer::new(msg_receiver))
        .serve(add)
        .await.unwrap();

    Ok(())
}