use std::io;

fn main () {
    loop {
    println!("Welcome to Brian's Compound Interest Savings Calculator ");

    println!("What is your principal amount? ");
    let mut p = String::new();
    io::stdin().read_line(&mut p).expect("Not a valid string ");
    let p:f64 = p.trim().parse().expect("Not a valid number ");

    println!("What is the rate? ");
    let mut r = String::new();
    io::stdin().read_line(&mut r).expect("Not a valid string ");
    let r : f64 = r.trim().parse().expect("Not a avlid number ");

    println!("How many years will you be saving for? ");
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Not a valid string ");
    let t:f64 = t.trim().parse().expect("Not a valid number ");

    let compound_interest = p * (1.0 + r / 100.0).powf(t);
    println!("The compound interest after {} years is {}", t, compound_interest);

    let amount:f64 = compound_interest + p;
    println!("The total amount after {} years is {}", t, amount);

    println!("Would you like to calculate for another customer? (yes/no)");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Not a valid string ");
    
    if response. == "yes" {
        continue;
    }  else if response == "no" {
        break;
        println!("Thank you for using Brian's Compound Interest Savings Calculator :D ");
    }
}

}