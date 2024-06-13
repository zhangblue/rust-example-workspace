use std::time::Duration;
use tonic::transport::Channel;
use crate::streaming_example::msg_api_client::MsgApiClient;
use crate::streaming_example::MsgRequest;

pub mod streaming_example {
    tonic::include_proto!("grpc.example.streaming");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MsgApiClient::connect("http://127.0.0.1:50051").await?;

    run_server_streaming_echo(&mut client, 10).await;
    Ok(())
}

// 模拟不停的向服务端发送数据
async fn run_server_streaming_echo(client: &mut MsgApiClient<Channel>, _num: usize) {
    // 创建模拟数据收集线程
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    let _data_collecter_task = tokio::spawn(async move {
        for i in 0..=1000 {
            let msg_request = MsgRequest {
                agent_id: format!("agent_id-{i}"),
                agent_type: format!("agent_type-{i}"),
                msg_data: format!("msg_data-{i}"),
            };

            println!("收集 {i}");
            let _ = tx.send(msg_request).await;
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    });


    // 创建模拟数据发送线程
    let outbound = async_stream::stream! {
        while let Some(request) = rx.recv().await{
            yield request;
        }
    };

    let _rep = client.server_streaming_echo(outbound).await;
}
