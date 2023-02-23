use std::io;

fn main() {
    println!("Simple Calculator");

    // Take input for first number
    let mut first_number = String::new();
    println!("Enter first number: ");
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line");

    // Take input for second number
    let mut second_number = String::new();
    println!("Enter second number: ");
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line");

    // Convert input strings to numbers
    let first_number: f64 = first_number.trim().parse().unwrap();
    let second_number: f64 = second_number.trim().parse().unwrap();

    // Take input for operator
    let mut operator = String::new();
    println!("Enter operator (+, -, *, /): ");
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");

    // Perform operation based on operator
    let result = match operator.trim() {
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "*" => first_number * second_number,
        "/" => first_number / second_number,
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    // Print result
    println!("Result: {}", result);
}