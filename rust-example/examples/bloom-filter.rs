use bloomfilter::Bloom;

fn main() {
    let mut exists_api_app_protocol: Bloom<String> = Bloom::new_for_fp_rate(50000000, 0.00001);

    println!("{}", exists_api_app_protocol.number_of_bits());
    println!("{}", exists_api_app_protocol.number_of_hash_functions());

    println!("{} kb", exists_api_app_protocol.number_of_bits() / 8 / 1024);
    println!(
        "{} mb",
        exists_api_app_protocol.number_of_bits() / 8 / 1024 / 1024
    );
}
