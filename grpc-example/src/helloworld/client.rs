use crate::hello_world::greeter_client::GreeterClient;
use crate::hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("grpc.example.helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}