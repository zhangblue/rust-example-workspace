#[tokio::main]
async fn main() {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_micros(1_000_000 / 500));
    let cc = chrono::Utc::now();
    let mut count = 0;
    loop {
        interval.tick().await;
        // println!("run......");
        count += 1;
        if count == 500 {
            let seconds = chrono::Utc::now().timestamp();
            println!("count====={}", seconds);
            count = 0;
        }
    }
}
