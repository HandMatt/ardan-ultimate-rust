#![warn(clippy::pedantic)]

#[allow(dead_code)]
fn ignore_me() {

}

fn main() {
    println!("Hello, world!");
    let numbers = (0..100).collect::<Vec<i32>>();
    for item in numbers {
        println!("{item}");
    }

    #[rustfmt::skip]
    mod section {
        const N:i32 = 1;
    }
}
