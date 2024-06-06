/// 用于测试字符串相似度算法
fn main() {
    let target = "{\"user_name\":\"李1\",\"age\":10,\"sex\":\"男\"}";
    let value = "{\"user_name\":\"李四\",\"age\":10,\"sex\":\"男\"}";

    levenshtein_test(target, value);
    hamming_test(&target, value);
}

// 计算字符串编辑距离
fn levenshtein_test(target: &str, value: &str) {
    println!("levenshtein = {}", levenshtein::levenshtein(target, value));
}

fn hamming_test(target: &str, value: &str) {
    if target.len() == value.len() {
        println!(
            "hamming distance = {}",
            hamming::distance(target.as_bytes(), value.as_bytes())
        );
    } else {
        println!("长度不相符，无法计算汉明距离");
    }
}

fn simhash_test(target: &str, value: &str) {
    println!("simhash 相似度： {}", simhash::similarity(target, value));
}

fn get_all_uri_list() -> Vec<String> {
    vec![
        "/v1/app/123/create".to_string(),
        "/v1/app/231/create".to_string(),
        "/v1/app/axa/create".to_string(),
        "/v1/app/asd/create".to_string(),
        "/v1/app/qsd/create".to_string(),
        "/v1/app/xxx/create".to_string(),
        "/v1/app/asd/create".to_string(),
        "/v1/app/3x2/create".to_string(),
        "/v1/app/3x2d-hahx-kwjd/show".to_string(),
        "/v1/app/3xx2-xjah-dkah/show".to_string(),
        "/v1/app/3xx2-xasd-123d/show".to_string(),
        "/v1/app/xasd-xxqh-xnah/show".to_string(),
        "/v1/app/l1u2-xmah-xngh/show".to_string(),
        "/v1/app/xxxx-1111-dddd/show".to_string(),
        "/v1/app/3x2/create/query/1001".to_string(),
        "/v1/app/3x2/create/query/1002".to_string(),
        "/v1/app/3x2/create/query/1003".to_string(),
        "/v1/app/3x2/create/query/1004".to_string(),
    ]
}
