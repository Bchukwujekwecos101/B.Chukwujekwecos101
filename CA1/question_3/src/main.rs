use std::io;

fn main () {
	println!("Welcome to Brian's Campus Cafe Order System");

	println!("====================");
	println!("| Code | Item     | Price(N) |");
	println!("-----------------------------|");
	println!("|  T   |  Tea     |  800     |");
	println!("|  C   |  Coffee  |  1,200   |");
	println!("|  S   | Sandwich |  2,000   |");
	println!("|  J   | Juice    |  1,500   |");
	println!("-----------------------------|");


	println!("What item code do you want? ");
	let mut item_code = String::new();
	io::stdin().read_line(&mut item_code).expect("Not a valid string ");


	println!("What quantity do you want? ");
	let mut item_quantity = String::new();
	io::stdin().read_line(&mut item_quantity).expect("Not a valid string ");
	let quantity:f64 = item_quantity().trim().parse().expect("Not a valid number ");

	let price_per_item = match item_code;

	let 




}