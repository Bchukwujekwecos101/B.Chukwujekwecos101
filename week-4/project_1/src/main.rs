// Rust Program to solve Quadratic equations and find roots
use std::io;

fn main() {
   
    println!("Welcome to Brian's Quadratic equation calculator :D! ");

    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("What is your value of a? ");
    io::stdin().read_line(&mut a).expect("Not a valid string ");
    let a:f64= a.trim().parse().expect("Not a vaid number");

    println!("What is your value of b? ");
    io::stdin().read_line(&mut b).expect("Not a valid string ");
    let b:f64 = b.trim().parse().expect("Not a valid number ");

    println!("What is your value of c? ");
    io::stdin().read_line(&mut c).expect("Not a valid string ");
    let c:f64 = c.trim().parse().expect("Not a valid number ");

    let discriminant = b * b - (4.0 * a * c);

    // Function to the roots of the quadratic equation
    let root1 = -b + discriminant.sqrt() / (2.0 * a);
    println!("The first root of this equation is: {}", root1 );
    
    //for negative root
    let root2 = -b - discriminant.sqrt() / (2.0 * a);
    println!("The second root of this equation is: {}", root2 );

    // Function to calculate the discriminant of the quadratic equation
    println!("The discriminant of this equation is: {}", discriminant );

    if discriminant > 0.0 {
        println!("This equation has two real roots :]! ");
    }
    
    else if discriminant == 0.0 {
        println!("This equation has exactly one real root! ");
    }

    else if discriminant < 0.0 {
        println!("This equation has no real roots :[ ")
    }
    
    
    // Function to handle complex roots
   let real_part =  2.0 * a;
   let imaginary_part = -discriminant.sqrt() / (2.0 * a);
    println!("The equation has no real roots");
    println!("The complex roots are: ");
    println!("Root 1 = {} + {}i", real_part, imaginary_part);
    println!("Root 2 = {} + {}i", real_part, imaginary_part);


    println!("Thank you for using Brian's Quadratic Equation Calculator! ");



}