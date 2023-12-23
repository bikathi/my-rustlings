use std::io;

fn main() {
    println!("WELCOME TO THIS SIMPLE CALCULATOR");

    println!("Enter your first number: ");
    let mut first_number: String = String::new();

    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read your input");

    let first_number: u32 = first_number.trim().parse().expect("Input is NaN");

    println!("Entered first_number is: {first_number}");
}
