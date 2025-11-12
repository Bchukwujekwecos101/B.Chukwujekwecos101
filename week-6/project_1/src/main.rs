 use std::io;

fn main() {
    
   println!("Welcome to Brian's Food ordering system:D ");

   println!("| ================================================   |");
   println!("| Menu Code |        Menu Item          | Price(NGN) |");
   println!(" ---------------------------------------------------- ");
   println!("|     P     | Poundo Yam/Edinkaiko Soup |   3200     |");
   println!("|     F     | Fried Rice & Chicken      |   3000     |");
   println!("|     A     | Amala & Ewedu Soup        |   2500     |");
   println!("|     E     | Eba & Egusi Soup          |   2000     |");
   println!("|     W     | White Rice & Stew         |   2500     |");
   println!(" ---------------------------------------------------- ");

   // Get item code from user
    println!("Enter menu code (P, F, A, E or W):");
    let mut menu_code = String::new();
    io::stdin().read_line(&mut menu_code).expect("Failed to read input");
    
    let menu_code = menu_code.trim().to_uppercase();

    // Get quantity from user
    println!("Enter quantity:");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    
    let quantity: u32 = quantity.trim().parse().expect("Please enter a valid number");

    // Calculate base price based on item code
    let price_per_item = match menu_code.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid menu code!");
            return;
        }
    };

    // Calculate total cost
    let total_cost = price_per_item * quantity;

    // Apply discount if total cost exceeds N10,000
    let final_amount = if total_cost > 10000 {
        let discount = (total_cost as f64) * 0.05;
        total_cost - discount as u32
    } else {
        total_cost
    };

    // Display results
    println!();
    println!("=== Order Summary ===");
    println!("Item: {}", get_item_name(&menu_code));
    println!("Quantity: {}", quantity);
    println!("Price per item: NGN{}", price_per_item);
    println!("Total cost: NGN{}", total_cost);
    
    if total_cost > 10000 {
        println!("Discount applied: 5%");
        println!("Discount amount: NGN{}", (total_cost as f64 * 0.05) as u32);
    }
    
    println!("Final amount payable: NGN{}", final_amount);

    // Helper function to get item name from code
fn get_item_name(code: &str) -> &str {
    match code {
        "P" => "Pounded Yam/Edinkaiko Soup",
        "F" => "Fried Rice & Chicken",
        "A" => "Amala & Ewedu Soup",
        "E" => "Eba & Egusi Soup",
        "W" => "White Rice & Stew",

        _ => "Unknown",
    }



}
















   



}
