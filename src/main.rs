mod fibonacci;

use clap::Parser;

#[derive(Parser)]
struct UserArgs {
    fibonacci_number: u32,
}

fn main() {
    let user_args = UserArgs::parse();
    let n = user_args.fibonacci_number;
    let mut fibonacci_solutions = fibonacci::FibonacciSolutions::new();

    match fibonacci_solutions.find_solution(n) {
        Ok(solution) => println!("Fibo of {} is {}", n, solution),
        Err(error) => println!("Fibonacci error: {}", error),
    }
}
