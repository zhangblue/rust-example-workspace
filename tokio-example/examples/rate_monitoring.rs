use std::time::Duration;

#[tokio::main]
async fn main() {
    tokio::spawn(do_job());

    tokio::time::sleep(Duration::from_secs(100)).await;
}

async fn do_job() {
    let mut rate_monitoring_counter = 0i64;
    let mut rate_monitoring_start = chrono::Utc::now().timestamp_millis();

    loop {
        rate_monitoring(&mut rate_monitoring_counter, &mut rate_monitoring_start).await;
        tokio::time::sleep(Duration::from_millis(100)).await;

        //println!("{}----{}", &rate_monitoring_counter, &rate_monitoring_start);
    }
}

async fn rate_monitoring(rate_monitoring_counter: &mut i64, rate_monitoring_start: &mut i64) {
    // 监控写入速度
    *rate_monitoring_counter += 1;

    let insert_speed_monitor_end = chrono::Utc::now().timestamp_millis();
    if insert_speed_monitor_end - *rate_monitoring_start >= 3_000 {
        println!(
            "insert kl_api_app to db speed : {} msg/s",
            *rate_monitoring_counter * 1_000 / (insert_speed_monitor_end - *rate_monitoring_start)
        );
        *rate_monitoring_start = insert_speed_monitor_end;
        *rate_monitoring_counter = 0;
    }
}
