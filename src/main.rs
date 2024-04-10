mod fibonacci;

use clap::Parser;

#[derive(Parser)]
struct UserArgs {
    fibonacci_number: u32,
}

fn main() {
    let user_args = UserArgs::parse();
    let n = user_args.fibonacci_number;
    println!("Fibonacci of {} is {}", n, fibonacci::fibonacci(n));
}
