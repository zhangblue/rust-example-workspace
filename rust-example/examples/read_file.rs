use sysinfo::System;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    println!("==={}", sys.total_memory());

    let data = lost_memory_check(&sys.total_memory());

    println!("{:?}======", data);
}

// 检查内存丢失告警
fn lost_memory_check(current_memory: &u64) -> Option<ResourcesUsageAlert> {
    let file_path = "/Users/zhangdi/Downloads/memory_total_mark";

    let file = std::path::Path::new(&file_path);

    if !file.exists() {
        if let Ok(mut file_create) = File::create(&file_path) {
            let _ = file_create.write_all(current_memory.to_string().as_bytes());
        }
        return None;
    }

    if let Ok(mut read_file) = File::open(&file_path) {
        let mut contents = String::new();
        if let Err(err) = read_file.read_to_string(&mut contents) {
            println!("读取文件失败, {}", err);
        }

        println!("===================={}", contents);

        match contents.trim().parse::<u64>() {
            Ok(value) => {
                println!("{}========{}", current_memory, value);

                if current_memory < &value {
                    return Some(ResourcesUsageAlert {
                        create_time: chrono::Utc::now().timestamp(),
                        rule_id: String::from("preset"),
                        rule_name: Some(String::from("内存丢失")),
                        suggest: Some(String::from("添加内存")),
                        monitor_type: Some(String::from("MEMORY")),
                    });
                };
            }
            Err(err) => {
                println!("解析失败! err = {}", err);
            }
        }
    }

    return None;
}

#[derive(Debug)]
pub struct ResourcesUsageAlert {
    pub create_time: i64,
    pub rule_id: String,
    pub rule_name: Option<String>,
    pub suggest: Option<String>,
    pub monitor_type: Option<String>,
}
