use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input error");

    let mut numbers = input.split_whitespace();

    let num1: i32 = numbers.next().unwrap().parse().expect("Invalid number");
    let num2: i32 = numbers.next().unwrap().parse().expect("Invalid number");

    let sum = num1 + num2;
    println!("{sum}");
}
