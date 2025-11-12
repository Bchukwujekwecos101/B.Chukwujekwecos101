use std::io;

fn main() {
    println!("Welcome to Brian's Electricity Bill Estimator!");

    // Get name
    println!("What is your name?");
    let mut customer_name = String::new();
    io::stdin().read_line(&mut customer_name).expect("Failed to read name");
    let customer_name = customer_name.trim(); // remove newline

    // Get units consumed
    println!("How many units did you consume?");
    let mut unit_consumption = String::new();
    io::std().read_line(&mut unit_consumption).expect("Failed to read units");
    
    let unit_consumption: f64 = unit_consumption.trim().parse().expect("Please enter a valid number");

    // Initialize rate and total
    let rate: f64;
    let mut total: f64;

    // Determine rate based on usage
    if unit_consumption <= 100.0 {
        rate = 20.0;
        total = unit_consumption * rate;
    } else if unit_consumption <= 300.0 {
        rate = 35.0;
        total = unit_consumption * rate;
    } else {
        rate = 50.0;
        total = unit_consumption * rate;
    }

    // Add surcharge for very high usage
    if unit_consumption > 500.0 {
        total += 5000.0;
    }

    // Print bill summary
    println!("\n=== EKDC Electricity Bill ===");
    println!("Customer Name: {}", customer_name);
    println!("Rate per unit: ₦{:.2}", rate);
    println!("Total units: {:.2}", unit_consumption);
    println!("Total bill: ₦{:.2}", total);
    println!("Thank you for using Brian's Electricity Bill Estimator! 😀");
}
