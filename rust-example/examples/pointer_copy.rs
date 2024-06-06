fn main() {
    let mut student_opt = Some(Student {
        name: "张三".to_string(),
    });

    update_memory(&mut student_opt);

    println!("{}", student_opt.unwrap().name);
}

#[derive(Clone, Debug)]
struct Student {
    name: String,
}

fn update_memory(student_opt: &mut Option<Student>) {
    let student_clone = student_opt.clone();

    let mut student = student_clone.unwrap();

    student.name = "李四".to_string();
}
