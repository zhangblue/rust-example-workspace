use std::{sync::RwLock, time::Duration};



#[tokio::main]
async fn main() {


    RwLock

    for i in 0..3 {
        let mut linked_list = Vec::new();

        linked_list.push("a".to_string());
        start_thread(i, linked_list).await;
    }
    println!("所有线程启动完成");
    tokio::time::sleep(Duration::from_secs(30)).await;
}

async fn start_thread(num: i64, list: Vec<String>) {
    println!("启动线程 [{}]", num);
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(3)).await;

        println!("thread [{num}] finish, list size = [{}]", list.len());
    });
}
