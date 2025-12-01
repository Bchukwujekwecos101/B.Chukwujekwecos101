use std::io;

fn main() {
    println!("Welcome to Brian's Shape Area and Volume Calculator :D! ");

    println!("------ Shape Calculator ------");
    println!("Please select one of the following options:");
    println!("1.          Area of Trapezium              ");
    println!("2.          Area of Rhombus                ");
    println!("3.          Area of Parallelogram          ");
    println!("4.          Area of Cube                   ");
    println!("5.          Volume of Cylinder             ");

    let mut choice = String::new();

    println!("Enter your choice (1-5): ");
    io::stdin().read_line(&mut choice).expect("Not a valid input");

    let choice: u8 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number between 1 and 5");
            return;
        }
    };

    match choice {
        1 => calculate_trapezium_area(),
        2 => calculate_rhombus_area(),
        3 => calculate_parallelogram_area(),
        4 => calculate_cube_area(),
        5 => calculate_cylinder_volume(),
        _ => println!("Invalid choice! Please select a number between 1 and 5."),
    }
}

fn calculate_trapezium_area() {
    println!("Calculating Area of Trapezium");

    let height = get_number_input("Enter the height: ");
    let base1 = get_number_input("Enter base 1: ");
    let base2 = get_number_input("Enter base 2: ");

    let area = 0.5 * height * (base1 + base2);
    println!("The area of the Trapezium = {:.2}", area);
}

fn calculate_rhombus_area() {
    println!("Calculating Area of Rhombus");

    let diagonal1 = get_number_input("Enter diagonal 1: ");
    let diagonal2 = get_number_input("Enter diagonal 2: ");

    let area = 0.5 * diagonal1 * diagonal2;
    println!("The area of the Rhombus = {:.2}", area);
}

fn calculate_parallelogram_area() {
    println!("Calculating Area of Parallelogram");

    let base = get_number_input("Enter the base: ");
    let altitude = get_number_input("Enter the altitude: ");

    let area = base * altitude;
    println!("The area of the Parallelogram = {:.2}", area);
}

fn calculate_cube_area() {
    println!("Calculating Area of Cube");

    let side = get_number_input("Enter the side length: ");

    let area = 6.0 * side * side;
    println!("The area of the Cube = {:.2}", area);
}

fn calculate_cylinder_volume() {
    println!("Calculating Volume of Cylinder");

    let radius = get_number_input("Enter the radius: ");
    let height = get_number_input("Enter the height: ");

    let volume = std::f64::consts::PI * radius * radius * height;
    println!("The volume of the Cylinder = {:.2}", volume);
}

fn get_number_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}