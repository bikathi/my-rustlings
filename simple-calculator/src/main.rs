use std::io;

fn main() {
    println!("WELCOME TO THIS SIMPLE CALCULATOR");

    println!("Enter your first number: ");
    let mut first_number: String = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read your input");

    let first_number: i32 = first_number.trim().parse().expect("Input is NaN");

    println!("Enter your second number: ");
    let mut second_number: String = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read second number");
    let second_number: i32 = second_number.trim().parse().expect("Input is NaN");

    println!("Enter your operator: ");
    let mut operator: String = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read operator");
    let operator = operator.trim().to_string();

    let result = compute_operation(first_number, second_number, operator);

    println!("Result is: {result}");
}

fn compute_operation(first_number: i32, second_number: i32, operator: String) -> i32 {
    if operator == "+" {
        first_number + second_number
    } else if operator == "-" {
        first_number - second_number
    } else if operator == "*" {
        first_number * second_number
    } else {
        first_number / second_number
    }
}
