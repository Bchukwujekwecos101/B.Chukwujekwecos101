// Simple struct to hold laptop brand and price
struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    // Method to calculate the cost of buying 'qty' laptops
    fn cost(&self, qty: u32) -> u32 {
        self.price * qty
    }
}

fn main() {
    // Creating laptop brands with their individual prices
    let hp = Laptop { brand: String::from("HP"), price: 650_000 };
    let ibm = Laptop { brand: String::from("IBM"), price: 755_000 };
    let toshiba = Laptop { brand: String::from("Toshiba"), price: 550_000 };
    let dell = Laptop { brand: String::from("Dell"), price: 850_000 };

    // Quantity customer buys from each brand
    let qty = 3;

    // Calculating total cost
    let total_cost =
          hp.cost(qty)
        + ibm.cost(qty)
        + toshiba.cost(qty)
        + dell.cost(qty);

    println!("Total cost for buying 3 from each brand is: ₦{}", total_cost);
}
