/// 多级借用
fn main() {
    demo1();
    println!("demo1 done .......");
    demo2();
    println!("demo2 done .......");
    demo3();
    println!("demo3 done .......");
}

fn demo1() {
    let mut a1 = 10u32;
    let mut b = &mut a1;
    *b = 20;

    println!("{b}");

    let c = &mut b;
    **c = 30;

    println!("{c}");
    println!("{a1}");
}

fn demo2() {
    let mut a1 = 10u32;
    let mut b = &mut a1;
    *b = 20;

    println!("{b}");

    let mut c = &mut b;

    **c = 30;
    println!("{c}");

    let d = &mut c;

    ***d = 40;

    println!("{d}");
}

fn demo3() {
    let mut a1 = 10u32;
    let mut b = &mut a1;
    let mut c = &mut b;
    let d = &mut c;

    ***d = 30;

    println!("{d}");
}
