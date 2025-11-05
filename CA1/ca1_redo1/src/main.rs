use std::io;

fn main() {
	println!("Welcome to Brian's Electricity Bill Estimator! ");

	println!("What is your name? ");
	let mut customer_name = String::new();
	io::stdin().read_line(&mut customer_name).expect("Not a valid string ");

	println!("How many units did you consume");
	let mut unit_consumption = String::new();
	io::stdin().read_line(&mut unit_consumption).expect("Not a valid string ");
	let unit_consumption: f64 = unit_consumption.trim().parse().expect("Not a valid number ");

	let rate: f64;

	let mut total: f64; 

	if unit_consumption <= 100.0 {
		rate = 20.0;
		total = unit_consumption * rate;
	} else if unit_consumption > 100.0 && unit_consumption <= 300.0 {
		rate = 35.0;
		total = unit_consumption * rate;
	} else if unit_consumption > 300.0 {
		rate = 50.0;
		total = unit_consumption * rate;
	}

	if unit_consumption > 500.0 {
		total += 5000.0;
	}
		

	println!(" === EKDC Electricity Bill === ");
	println!("Customer's Name: {}", customer_name);
	println!("The rate is: {:.2}", rate);
	println!("The total units are: {:.2}",unit_consumption);
	println!("Your total bill is: {:.2}", total);
	println!("Thank you for using Brian's Electricity Bill Estimator :D !");

}