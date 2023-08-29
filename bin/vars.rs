pub mod sub_a;
pub mod sub_b;

pub fn run() {
    println!("Here is vars module!");

    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;  // RustではDefaultで全ての変数がImmutable
    let mut x = 5; // mutをつけることで可変な変数として定義可能
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
