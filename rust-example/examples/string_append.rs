extern crate core;

use core::fmt;
use std::fmt::Formatter;

fn main() {
    let mut vec = vec![];

    vec.push(AnomalyInfo { code: 0, name: String::from("张0") });
    vec.push(AnomalyInfo { code: 1, name: String::from("张1") });
    vec.push(AnomalyInfo { code: 2, name: String::from("张2") });
    vec.push(AnomalyInfo { code: 3, name: String::from("张3") });
    vec.push(AnomalyInfo { code: 4, name: String::from("张4") });

    let cc = vec.into_iter().map(|info| info.format_value())
        .collect::<Vec<_>>()
        .join(",");

    println!("{cc}");
}

#[derive(Debug)]
struct AnomalyInfo {
    code: u8,
    name: String,
}

impl fmt::Display for AnomalyInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}]", self.code, self.name)
    }
}

impl AnomalyInfo {
    fn format_value(&self) -> String {
        format!("[{}:{}]", self.code, self.name)
    }
}