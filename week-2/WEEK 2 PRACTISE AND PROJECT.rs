// PRACTISE 1
fn main(){
    println!("Welcome to COS 101");
	
	println!("The course learning outcome is as follows:");
	
	println!("1. Distinguish between computer and computer programming,/n
	          2. Develop some techniques in computer science,/n
			  3. Understand the different areas of study in computer science,/n
			  4. Conversant with appications of computer science,/n
			  5. Navigate the career prospects in computer science,/n
			  6. Conversant with computer programming concepts,");
  }



//PRACTISE 2
fn main(){
    println!(); // prints just a
	println!("hello ");//prints
	println!("format {} arguments", "some");//prints format some arguments
	println!("My name is {}. I am the class rep of stream 1", "Brian Chukwujekwe");
	
}
	
	

//PRACTISE 3
fn main() {
   let fees = "25000";
   let salary:f64 = 35_000.00;
   println!("fees is {} and salary is {}", fees, salary);
   
}


//PRACTISE 4
fn main() {
  let p:f64 = 1000.0;
  let r:f64 = 1.0;
  let t:f64 = 2.0;
  
  // simple interest
  let a = p * (1.0 + (r / 100.0)) * t;
  println!("Amount is {}", a);
  let si = a - p;
  println!("Simple Interest is {}" , si)
  
}
   

//PROJECT 1  
fn main() {
    let p:f64 = 520000000.0; // Principal amount
    let r:f64 = 10.0;        // Rate per annum
    let n:f64 = 5.0;         // Time in years

    // Formula: A = P * (1 + R/100)^n
    let a = p * (1.0 + r / 100.0).powf(n);

    // Compound Interest = A - P
    let ci = a - p;

    println!("Compound Interest = ₦{}", ci);
	
}


//PROJECT 2
fn main() {
    let a: f64 = 450000.0;  // Toshiba amount
    let b: f64 = 1500000.0; // Mac amount
    let c: f64 = 750000.0;  // HP amount
    let d: f64 = 2850000.0; // Dell amount
    let e: f64 = 250000.0;  // Acer amount

    // Formula: sum = a + b + c + d + e
    let sum = a + b + c + d + e;
    println!("sum = {} sales", sum);

    // Formula: average = sum / 5
    let average = sum / 5.0;
    println!("average = {} is the average", average);
}


//PROJECT 3
fn main() {
    let p: f64 = 510000.0; // Principal amount
    let r: f64 = 5.0; // Rate per annum
    let n: f64 = 3.0; // Time in years
    
    // Formula: A = P[1-(R/100)]^n
    let a = p * (1.0 - (r / 100.0)).powf(n);
    println!("After 3 years the value of the tv is ₦{:.2}", a);
}