// a simple grader that asks the user to enter number of subjects to grade, enter the subject and the mark and
// assigns grades and does an average, displaying the final output in a user-friendly manner

use std::io;

fn main() {
    let mut subject_number: String = String::new();
    let mut subject_list: Vec<i32> = vec![];
    let mut marks_list: Vec<i32> = vec![];

    println!("How many subjects do you want to grade?: ");
    io::stdin()
        .read_line(&mut subject_number)
        .expect("Unable to read number of subjects");

    let mut subject_number: i32 = subject_number.trim().parse().expect("Input is NaN");

    take_subject_input(&mut subject_number, &mut subject_list, &mut marks_list);
}

fn take_subject_input(counter: &mut i32, sub_list: &mut Vec<i32>, mark_list: &mut Vec<i32>) {
    println!("Inside the take_subject_input function");
    while *counter > 0 {
        // take each subject and marks for the subject pushing into the relevant vec!
        println!("Enter {} subject name: ", *counter);
        let mut subject_name: String = String::new();
        io::stdin()
            .read_line(&mut subject_name)
            .expect("Unable to read subject name");
        let subject_name: &str = subject_name.trim();

        println!("Enter {} marks: ", subject_name);
        let mut subject_mark: String = String::new();
        io::stdin()
            .read_line(&mut subject_mark)
            .expect("Unable to read subject marks");
        let subject_mark: i32 = subject_mark.trim().parse().expect("Mark is NaN");
        println!("Student has {} marks in {}", subject_mark, subject_name);

        *counter -= 1;
    }
}
