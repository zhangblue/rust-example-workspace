use trie_rs::TrieBuilder;

fn main() {
    let all_apis = get_all_uri_list();

    trie_tree(&all_apis);

    // let size = all_apis.len();

    // let mut i = 1;
    // while i < size {
    //     levenshtein_test(&all_apis.get(i - 1).unwrap(), &all_apis.get(i).unwrap());
    //     i += 1;
    // }
}

// 计算字符串编辑距离
fn levenshtein_test(target: &str, value: &str) {
    println!(
        "value1 = {}, value2 = {}, levenshtein = {}",
        target,
        value,
        levenshtein::levenshtein(target, value)
    );
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
    println!(
        "simhash value1 = {}, value2 = {},  相似度： {}",
        target,
        value,
        simhash::similarity(target, value),
    );
}

fn trie_tree(_uri_list: &Vec<String>) {
    let mut builder = TrieBuilder::new();
    builder.push(vec!["a", "woman"]);
    builder.push(vec!["a", "woman", "on", "the", "beach"]);
    builder.push(vec!["a", "woman", "on", "the", "run"]);

    let trie = builder.build();

    println!(
        "exact_match = {}",
        trie.exact_match(vec!["a", "woman", "on", "the", "beach"])
    );

    let r: Vec<Vec<&str>> = trie.predictive_search(vec!["a", "woman", "on"]).collect();

    println!("predictive_search = {:?}", r);

    let s: Vec<Vec<&str>> = trie
        .common_prefix_search(vec!["a", "woman", "on", "the", "beach"])
        .collect();

    println!("common_prefix_search = {:?}", s);
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
