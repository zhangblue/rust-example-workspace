//! 判断字符串是否是ip或者域名

use regex::Regex;

fn main() {
    println!(
        "value is = {}",
        check_content_has_special_character("123pwd")
    );
}

fn check_content_has_special_character(content: &str) -> bool {
    let regex_value = String::from(r"[\\/]");

    let regex_o = Regex::new(&regex_value).unwrap();

    regex_o.is_match(content)
}

fn is_ip_or_host(content: &str) -> bool {
    let regex_value = String::from(
        r"^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$|^((?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}|(?:\d{1,3}\.){3}\d{1,3})$",
    );

    let regex_o = Regex::new(&regex_value).unwrap();

    regex_o.is_match(content)
}
