use std::io;
use std::vec;

/// Assignment object used for all assignments
#[derive(Debug)]
struct Assignment {
    label: String,
    grade: Option<f32>,
    points: i32,
}

/// Calculates the total average in the class based on assignments
fn calculate_total_average(assignment_list: Vec<Assignment>) -> f32 {
    let mut point_total = 0f32;
    let mut student_points = 0f32;

    for assignment in assignment_list {
        if assignment.grade.is_none() {
            continue;
        }

        point_total += assignment.points as f32;
        student_points += assignment.grade.unwrap() * assignment.points as f32;
    }

    student_points / point_total
}

fn main() {
    let test1 = Assignment {
        label: "Test 1".to_string(),
        grade: None,
        points: 110,
    };
    let test2 = Assignment {
        label: "Test 2".to_string(),
        grade: None,
        points: 110,
    };
    let test3 = Assignment {
        label: "Test 3".to_string(),
        grade: None,
        points: 110,
    };

    let final_exam = Assignment {
        label: "Final Exam".to_string(),
        grade: None,
        points: 120,
    };

    let homework = Assignment {
        label: "Homework".to_string(),
        grade: None,
        points: 50,
    };
    let quizzes = Assignment {
        label: "Quizzes".to_string(),
        grade: None,
        points: 50,
    };

    println!("Calculus Gradinator\nEnter grade for corresponding assignment or return if N/A");

    let mut assignment_list = vec![test1, test2, test3, final_exam, homework, quizzes];

    for mut assignment in assignment_list.iter_mut() {
        println!("Enter average for {}", assignment.label);

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        assignment.grade = match input.trim().parse::<f32>() {
            Ok(i) => Some(i / 100f32),
            Err(_) => None,
        };
    }

    println!(
        "Your class average is {:.2}%",
        calculate_total_average(assignment_list) * 100f32
    );
}
