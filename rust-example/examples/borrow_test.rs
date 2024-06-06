fn main() {
    let mut point = Point { x: 1, y: 2 };

    let result = borrow_fun(&mut point.x, &mut point.y);

    println!("result = {}", result);
}

fn borrow_fun(x: &mut i32, y: &mut i32) -> i32 {
    *x = *x + 1;
    *y = *y + 1;

    return *x * *y;
}

struct Point {
    x: i32,
    y: i32,
}
