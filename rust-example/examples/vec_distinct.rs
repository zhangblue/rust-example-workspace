mod display_impl;

use std::collections::HashSet;

/// 将vec中的数据去重
fn main() {
    let vec_with_duplicates = vec![1, 2, 3, 4, 4, 5, 6, 6, 7];

    let distinct_value: Vec<i32> = vec_with_duplicates
        .into_iter()
        .map(|x| x)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    println!("{:?}", distinct_value);
}
