use std::io;

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("Division by zero");
    }
    a / b
}

fn main() {
    println!("Simple Calculator");

    let mut num1 = String::new();
    println!("Enter the first number:");
    io::stdin().read_line(&mut num1).expect("Failed to read input");
    let num1: f64 = num1.trim().parse().expect("Invalid number");

    let mut num2 = String::new();
    println!("Enter the second number:");
    io::stdin().read_line(&mut num2).expect("Failed to read input");
    let num2: f64 = num2.trim().parse().expect("Invalid number");

    println!("Choose an operation:");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = choice.trim().parse().expect("Invalid choice");

    let result = match choice {
        1 => add(num1, num2),
        2 => subtract(num1, num2),
        3 => multiply(num1, num2),
        4 => divide(num1, num2),
        _ => {
            println!("Invalid choice");
            return;
        }
    };

    println!("Result: {}", result);
}
