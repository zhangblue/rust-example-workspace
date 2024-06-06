use std::path::Path;

use md5::Digest;
fn main() {
    let path = Path::new("/Users/zhangdi/Downloads").join("value");

    if !path.exists() {
        std::fs::create_dir_all(path);
    }
}
