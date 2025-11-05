use std::io;

fn main() {
    // Display the menu
    println!("Computer Store Inventory");
    println!("=========================");
    println!("| Code | Item     | Price (NGN) |");
    println!("|------|----------|-------------|");
    println!("| L    | Laptop   | 550,000     |");
    println!("| M    | Monitor  | 120,000     |");
    println!("| K    | Keyboard | 15,000      |");
    println!("| H    | Headset  | 25,000      |");
    println!();

    // Get item code from user
    println!("Enter item code (L, M, K, or H):");
    let mut item_code = String::new();
    io::stdin().read_line(&mut item_code).expect("Failed to read input");
    
    let item_code = item_code.trim().to_uppercase();

    // Get quantity from user
    println!("Enter quantity:");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    
    let quantity: u32 = quantity.trim().parse().expect("Please enter a valid number");

    // Calculate base price based on item code
    let price_per_item = match item_code.as_str() {
        "L" => 550000,
        "M" => 120000,
        "K" => 15000,
        "H" => 25000,
        _ => {
            println!("Invalid item code!");
            return;
        }
    };

    // Calculate total cost
    let total_cost = price_per_item * quantity;

    // Apply discount if total cost exceeds ¥500,000
    let final_amount = if total_cost > 500000 {
        let discount = (total_cost as f64) * 0.07;
        total_cost - discount as u32
    } else {
        total_cost
    };

    // Display results
    println!();
    println!("=== Order Summary ===");
    println!("Item: {}", get_item_name(&item_code));
    println!("Quantity: {}", quantity);
    println!("Price per item: ¥{}", price_per_item);
    println!("Total cost: ¥{}", total_cost);
    
    if total_cost > 500000 {
        println!("Discount applied: 7%");
        println!("Discount amount: NGN{}", (total_cost as f64 * 0.07) as u32);
    }
    
    println!("Final amount payable: NGN{}", final_amount);
}

// Helper function to get item name from code
fn get_item_name(code: &str) -> &str {
    match code {
        "L" => "Laptop",
        "M" => "Monitor",
        "K" => "Keyboard",
        "H" => "Headset",
        _ => "Unknown",
    }
}