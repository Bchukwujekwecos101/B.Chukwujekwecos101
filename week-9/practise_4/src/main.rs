use std::io;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {

	println!("Please indicate your name and add a message to the file: ");
	let added_information = String::new();
	io::stdin().read_line(&mut added_information).expect("Input failed");
	let added:&str = &added_information;


	let mut file = OpenOptions::new().append(true).open("data.txt").expect(
		"cannot open file");
	file.write_all("\nHello Class".as_bytes()).expect("write failed");
	file.write_all("\nThis is the appendage to the document."
		.as_bytes()).expect("write failed");
	println!("file appendage success");
}