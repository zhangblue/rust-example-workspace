use regex::Regex;
use url::Url;

fn main() {
    let url_str = "https://jira.cyberkl.com/secure/Dashboard.jspa";

    match extract_root_domain(&url_str) {
        Some(data) => {
            println!("yes ===={}", data);
        }
        None => {
            println!("no ====");
        }
    }
}

// 提取一级域名，如果没有一级域名或者发生错误，则返回参数本身
fn extract_root_domain(full_domain: &str) -> Option<String> {
    let value = if full_domain.starts_with("http") {
        full_domain.to_string()
    } else {
        format!("{}{}", "http://", full_domain)
    };

    if let Ok(url) = Url::parse(&value) {
        if let Some(data) = url.host_str() {
            let re = Regex::new(r"([a-z0-9-]+\.(?:[a-z]{2,6}|[a-z]{2}\.[a-z]{2}))$").unwrap();
            return re.captures(data).map_or_else(
                || Some(full_domain.to_string()),
                |caps| caps.get(1).map(|m| m.as_str().to_string()),
            );
        }
    }

    return Some(full_domain.to_string());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn replate_colon() {
        let mut app_addr = String::from("lbytest.gmp.csg.cn:8089");

        if let Some(index) = app_addr.rfind(':') {
            app_addr.replace_range(index.., "");
        }

        println!("app_addr = {}", app_addr);
    }
}
