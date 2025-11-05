use std::io;

fn main() {
    
    	println!("Welcome to Brian's Student Loan Repayment Estimator");

    	println!("What is your value for p? ");
    	let mut input1 = String::new();
    	io::stdin().read_line(&mut input1).expect("Not a valid string ");
    	let p:f64 = input1.trim().parse().expect("Not a valid number ");

    	println!("What is your annual interest rate? ");
    	let mut input2 = String::new();
    	io::stdin().read_line(&mut input2).expect("Not a valid string");
    	let r:f64 = input2.trim().parse().expect("Not a valid number ");

    	println!("How many years have you borrowed for? ");
    	let mut input3 = String::new();
    	io::stdin().read_line(&mut input3).expect("Not a valid string");
    	let t:f64 = input3.trim().parse().expect("Not a valid number ");

    	let amount:f64 = p * (1.0 + r / 100.0).powf(t);
    	println!("The amount after {} years is {}", t, amount);

    	let local_interest:f64 = amount - p;
    	println!("The local interest after {} years is {}", t, local_interest);

    	println!("Would you like to calculate for another borrower? (y/n) ");
    	let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Not a valid string");

        if response == "y" {
        	main()
        } else if response == "n"{
        	println!("Thank you for using Brian's Student Loan Repayment Estimator:D ");
        }
        
        
    	
    }
    

