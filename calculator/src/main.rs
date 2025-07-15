use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: cargo run <add|sub> <num1> <num2>");
        process::exit(1);
    }

    let operation = &args[1];
    let num1: f64 = args[2].parse().unwrap_or_else(|_| {
        eprintln!("err {} is not a valid number.", args[2]);
        process::exit(1);
    });

    let num2: f64 = args[3].parse().unwrap_or_else(|_| {
        eprintln!("err {} is not a valid number.", args[3]);
        process::exit(1);
    });

    let result = match operation.as_str() {
        "add" => num1 + num2,
        "sub" => num1 - num2,
        _ => {
            eprintln!("unsupported '{}'use either 'add' or 'sub'.", operation);
            process::exit(1);
        }
    };

    println!("Result: {}", result);
}
