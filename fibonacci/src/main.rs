use std::io;
use std::io::Write;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn main() {
    println!("Fibonacci Series !");

    print!("Enter n => ");

    io::stdout().flush().unwrap();

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Unable to Read Input.");

    let n = n.trim().parse().expect("Invalid Number");

    println!("The term {} of fibonacci series is {}", n, fibonacci(n));
}
