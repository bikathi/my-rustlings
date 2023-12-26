// a simple grader that asks the user to enter number of subjects to grade, enter the subject and the mark and
// assigns grades and does an average, displaying the final output in a user-friendly manner

use std::{io, vec};

fn main() {
    let mut subject_number: String = String::new();
    let mut subject_list: Vec<String> = vec![];
    let mut marks_list: Vec<i32> = vec![];

    println!("How many subjects do you want to grade?: ");
    io::stdin()
        .read_line(&mut subject_number)
        .expect("Unable to read number of subjects");

    let mut subject_number: i32 = subject_number.trim().parse().expect("Input is NaN");

    take_subject_input(&mut subject_number, &mut subject_list, &mut marks_list);
    let grades_list: Vec<char> = calculate_grade(&mut marks_list);
    display_result(&subject_list, &marks_list, &grades_list);
}

fn take_subject_input(counter: &mut i32, sub_list: &mut Vec<String>, mark_list: &mut Vec<i32>) {
    while *counter > 0 {
        // take each subject and marks for the subject pushing into the relevant vec!
        println!("Enter {} subject name: ", *counter);
        let mut subject_name: String = String::new();
        io::stdin()
            .read_line(&mut subject_name)
            .expect("Unable to read subject name");
        let subject_name = subject_name.trim();
        sub_list.push(String::from(subject_name));

        println!("Enter {} marks: ", subject_name);
        let mut subject_mark: String = String::new();
        io::stdin()
            .read_line(&mut subject_mark)
            .expect("Unable to read subject marks");
        let subject_mark: i32 = subject_mark.trim().parse().expect("Mark is NaN");
        mark_list.push(subject_mark);

        *counter -= 1;
    }
}

fn calculate_grade(mark_list: &mut Vec<i32>) -> Vec<char> {
    let mut grades: Vec<char> = vec![];
    for mark in mark_list {
        let grade: char = if *mark >= 40 {
            'A'
        } else if *mark >= 30 && *mark < 40 {
            'B'
        } else if *mark >= 20 && *mark < 30 {
            'C'
        } else {
            'D'
        };

        grades.push(grade);
    }

    grades
}

fn display_result(sub_list: &Vec<String>, mark_list: &Vec<i32>, grade_list: &Vec<char>) {
    println!("===== Subject ===== Marks ===== Grade =====");
    let mut number_of_subjects: usize = sub_list.len();

    while number_of_subjects > 0 {
        println!(
            "      {}       {}           {}     ",
            sub_list[number_of_subjects - 1],
            mark_list[number_of_subjects - 1],
            grade_list[number_of_subjects - 1]
        );

        number_of_subjects -= 1;
    }
}
