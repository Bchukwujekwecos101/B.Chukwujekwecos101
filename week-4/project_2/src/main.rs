// Rust program to determine annual incentives based on age and experience of workers
use std::io;

fn main() {

    println!("Welcome to Brian's annual incentive calculator :D! ");
    
    // Input for user's name
    println!("What is your name? ");
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name).expect("Not a valid string ");
    
    // Input for user experience
    println!("Are you experienced? (yes/no): ");
    let mut user_experience = String::new();
    io::stdin().read_line(&mut user_experience).expect("Not a valid string ");

    // Input for user age
    println!("How old are you? ");
    let mut user_age = String::new();
    io::stdin().read_line(&mut user_age).expect("Not a valid string ");
    let user_age: u8 = user_age.trim().parse().expect("Not a valid number ");

     // Determine incentive
    let incentive: f64;

    if experienced {
        if user_age >= 40 {
            incentive = 1_560_000.0;
        
        } else if user_age >= 30 && user_age < 40 {
            incentive = 1_480_000.0;
        } else if user_age < 28 {
            incentive = 1_300_000.0 * 12.0;
        } else {
            incentive = 1_300_000.0;
        }
    } else {
        incentive = 100_000.0;
    }

    println!("Employee name: {}", user_name );
    println!("Employee experience: {}", if experienced { "Experienced" } else { "Inexperienced" });
    println!("Employee age: {}", user_age);
    println!("Annual Incentive: ₦{:.2}", incentive);
    println!("=====================================");
    println!("Thank you for using Brian's Incentive Calculator!");


}