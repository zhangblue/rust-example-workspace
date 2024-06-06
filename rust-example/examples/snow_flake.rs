use snowflake_rs::SnowFlakeId;

fn main() {
    let mut falke_id = SnowFlakeId::new(1, snowflake_rs::STANDARD_EPOCH);

    for i in 0..1000000 {
        let id = falke_id.generate_id().unwrap().to_string();
        if id.is_empty() {
            println!("empty = {}", i);
        }
    }

    let value = String::from(" ");

    println!("==={}", value.is_empty());
}
