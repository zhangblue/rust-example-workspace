use amiquip::{Channel, Connection};
use anyhow::Ok;

#[tokio::main]
async fn main() {
    // send_msg_to_mq().await;

    create_consumer().await;
}

async fn create_consumer() {}

async fn send_msg_to_mq() {}

async fn get_channel() -> anyhow::Result<Channel> {
    let addr = "amqp://cyberkl_user:cyberkl_password@172.16.103.10:5672";

    let mut connection = Connection::insecure_open(addr)?;

    let channel = connection.open_channel(None)?;

    Ok(channel)
}
