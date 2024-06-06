use regex::Regex;
use uri_pattern_matcher::UriPattern;

fn main() {
    // use_rex();
    println!("-------");
    match_test();
    // regex_match();
}

fn match_test() {
    let uri_tem = "/gmp/mm/";
    let uri_match = "/gmp/mm/123";
    if uri_tem.split('/').count() == uri_match.split('/').count() {
        let v = format!("{}{}", uri_tem, "{param}");
        println!("is match = [{}]", match_func(&v, uri_match));
    } else {
        println!("长度不一样");
    }
}

fn match_func(template: &str, uri: &str) -> bool {
    if template.split('/').count() == uri.split('/').count() {
        let pattern: UriPattern = template.into();
        return pattern.is_match(uri);
    }

    return false;
}

fn use_rex() {
    let regex_value = String::from(
        r"(/gmp/mm/supplier-outer/eTemplateClassQuery/saveSelfEvaluationBusinessInfo/)([^/\s]*?)(/)([^/\s]*?)(/)([^/\s]*?)(/)([^/\s]*)$",
    );

    let regex_o = Regex::new(&regex_value).unwrap();

    let value = String::from(
        r"/gmp/mm/supplier-outer/eTemplateClassQuery/saveSelfEvaluationBusinessInfo/0/0024FB865F374209BEA53AA4E1111128/123/123",
    );
    let begin = chrono::Utc::now().timestamp_millis();

    for i in 0..100000 {
        regex_o.is_match(&value);
    }

    let end = chrono::Utc::now().timestamp_millis();

    println!("{}", end - begin);
}

fn use_uri_macher() {
    let pattern: UriPattern = "/gmp/mm/supplier-outer/eTemplateClassQuery/saveSelfEvaluationBusinessInfo/0/0024FB865F374209BEA53AA4E1111128/123/123/".into();

    let str: &str = "/gmp/mm/supplier-outer/eTemplateClassQuery/saveSelfEvaluationBusinessInfo/0/0024FB865F374209BEA53AA4E1111128/123/123/";

    let begin = chrono::Utc::now().timestamp_millis();

    // for i in 0..100000 {
    // str.split('/');
    println!("match = {}", pattern.is_match(str));
    // }

    let end = chrono::Utc::now().timestamp_millis();

    println!("{}", end - begin);
}

fn regex_match() {
    let regex_value = String::from(r"(.*)/([0-9a-zA-Z]{3,5})/(children)/([0-9a-zA-Z]{2,7})");

    let regex_o = Regex::new(&regex_value).unwrap();

    let value = String::from(r"asdx/example/12xxxx3/children/a2dasd");

    println!("match result = [{}]", regex_o.is_match(&value));
}
