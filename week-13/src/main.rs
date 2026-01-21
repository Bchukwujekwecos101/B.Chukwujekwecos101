use std::io::Read;
use std::io;

fn admin() {
    let mut file = std::fs::File::open("globcom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}
fn pro_mgr() {
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}
fn employee() {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}
fn customer() {
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}
fn vendor() {
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}

fn main() {

    let mut input1 = String::new();
    
    println!("administrator = A");
    println!("project manager = P");
    println!("employee = E");
    println!("customer =  C");
    println!("vendor = V");
    println!("Please enter your role in the company");
    io::stdin().read_line(&mut input1).expect("Unable to read input");
    let input1 = input1.trim().to_uppercase();

if input1 == "A"  {
    admin();
}else if input1 == "P"{
    pro_mgr();
}else if input1 == "E" {
    employee();
}else if input1 == "C"{
    customer();
}else if input1 == "V" {
    vendor();
}else {
    println!("Sorry role not availabele");
}

}
    
