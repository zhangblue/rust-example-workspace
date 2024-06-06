use std::fmt::Display;

/// 实现 Display trait
fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("{matrix}");
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}
