use std::ptr::eq;

fn main() {
    let student = Student {
        name: Some("张三".to_string()),
    };


}

fn equals_name(student: &Student) {
    let new_name = match student.name.as_ref(){
        Some(data) => data,
        None => "",
    };
}

struct Student {
    name: Option<String>,
}
