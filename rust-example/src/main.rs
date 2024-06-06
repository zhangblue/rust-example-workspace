fn main() {
    let cc = chrono::Utc::now().naive_utc();

    let seconds = cc.and_utc().timestamp();
    println!("{cc:?}-----{seconds}");
}
