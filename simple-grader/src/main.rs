// a simple grader that asks the user to enter number of subjects to grade, enter the subject and the mark and
// assigns grades and does an average, displaying the final output in a user-friendly manner

use std::io;

fn main() {
    let mut subject_number: String = String::new();

    println!("How many subjects do you want to grade?: ");
    io::stdin()
        .read_line(&mut subject_number)
        .expect("Unable to read number of subjects");

    let mut subject_number: i32 = subject_number.trim().parse().expect("Input is NaN");
    println!("Entered subjects {subject_number}");
}
