use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    types::{AMQPValue, FieldTable, LongLongInt, ShortString},
    BasicProperties, Channel, Connection, ConnectionProperties,
};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    send_msg_to_mq().await;

    create_consumer().await;
}

async fn create_consumer() {
    let channel = get_channel().await.unwrap();

    let mut consumer = channel
        .basic_consume(
            "test_data_queue",
            "test_data_queue_consumer_tag",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("basic_consume");

    let basic_ack_options = BasicAckOptions { multiple: true };

    // let basic_ack_options = BasicAckOptions::default();

    println!("current ack = [{}]", basic_ack_options.multiple);

    for i in 0..10 {
        match consumer.next().await {
            Some(Ok(delivery)) => {
                if let Ok(msg_str) = std::str::from_utf8(&delivery.data) {
                    println!("consumer data = {}", msg_str);
                }

                if i == 3 {
                    delivery.ack(basic_ack_options).await.expect("basic_ack");
                }
            }
            Some(Err(e)) => {
                eprint!("consumer error = [{}]", e);
            }

            None => {
                eprint!("consumer is none");
            }
        }
    }
}

async fn send_msg_to_mq() {
    let channel = get_channel().await.unwrap();

    for i in 0..100 {
        let msg = String::from("a");
        String::from("a").push_str(&i.to_string());

        let result = channel
            .basic_publish(
                "",
                "test_data_queue",
                BasicPublishOptions::default(),
                msg.as_bytes(),
                BasicProperties::default(),
            )
            .await;

        match result {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

async fn get_channel() -> anyhow::Result<Channel> {
    let addr = "amqp://cyberkl_user:cyberkl_password@172.16.103.10:5672";
    let conn = Connection::connect(addr, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    let mut filed_table = FieldTable::default();
    filed_table.insert(
        ShortString::from("x-max-length-bytes"),
        AMQPValue::LongLongInt(10_000_000_000 as LongLongInt),
    );
    channel
        .queue_declare(
            "test_data_queue",
            QueueDeclareOptions::default(),
            filed_table,
        )
        .await?;
    Ok(channel)
}
