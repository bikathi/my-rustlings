use std::io;

fn main() {
    println!("WELCOME TO THIS SIMPLE CALCULATOR");

    println!("Enter your first number: ");
    let mut first_number: String = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read your input");

    let first_number: u32 = first_number.trim().parse().expect("Input is NaN");

    println!("Enter your second number: ");
    let mut second_number: String = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read second number");
    let second_number: u32 = second_number.trim().parse().expect("Input is NaN");

    let mut operator: String = String::new();
    println!("Enter your operator: ");
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read entered operator");

    println!("Entered first_number is: {first_number}");
    println!("Entered second_number is: {second_number}");
    println!("Entered operator is: {operator}");
}
