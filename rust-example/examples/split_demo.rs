fn main() {
    let s = "foobarbaz";

    // 使用 split 方法根据 '?' 进行分割
    let mut parts = s.split('?');

    // 使用 next 方法获取拆分后的第一个元素
    if let Some(first) = parts.next() {
        println!("第一个元素是: {}", first);
    } else {
        println!("没有找到分隔符 '?'");
    }
}
