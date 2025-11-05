use std::io;

fn main () {

    println!("Welcome to Brian's Compound Interest Savings Calculator ");

    println!("What is your name?" );
    let mut student_name = String::new();
    io::stdin().read_line(&mut student_name).expect("Not a valid string");

    println!("What is the score of your first? ");
    let mut test_score1 = String::new();
    io::stdin().read_line(&mut test_score1).expect("Not a valid string" );
    let test_score1:f64 = test_score1.trim().parse().expect("Not a valid number");

    println!("What is the score your second test? ");
    let mut test_score2 = String::new();
    io::stdin().read_line(&mut test_score2).expect("Not a valid string");
    let test_score2:f64 = test_score2.trim().parse().expect("Not a valid number");

    println!("What is the score of the third test?");
    let mut test_score3 = String::new();
    io::stdin().read_line(&mut test_score3).expect("Not a valid string");
    let test_score3:f64 = test_score3.trim().parse().expect("Not a valid number");

    let average_score:f64 = (test_score1 as f64 + test_score2 as f64 + test_score3 as f64) / 3.0;
    
    if average_score >= 70.0 {
    println!("Student Name: {}", student_name.trim());
    println!("Student Score: {:.2}", average_score);
    println!("Student Grade: A ");
    
    } else if average_score >= 60.0 && average_score < 70.0 {
    println!("Student Name: {}", student_name.trim());
    println!("Student Score: {:.2}", average_score);
    println!("Student Grade: B ");
    
    }else if average_score >= 50.0 && average_score < 60.0{
    println!("Student Name: {}", student_name.trim());
    println!("Student Score: {:.2}", average_score);
    println!("Student Grade: C ");

    }else if average_score >= 45.0 && average_score < 50.0 {
    println!("Student Name: {}", student_name.trim());
    println!("Student Score: {:.2}", average_score);
    println!("Student Grade: D ");

    }else if average_score < 45.0 {
    println!("Student Name: {}", student_name.trim());
    println!("Student Score: {:.2}", average_score);
    println!("Student Grade: F ");
    }
    
    println!("Thank you for using Brian's Student Grade Evaluator! ");


}