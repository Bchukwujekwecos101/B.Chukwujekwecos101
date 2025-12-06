use std::fs::File;
use std::io::Write;

fn main() {
    // Arrays for datasets 
    let names = [
    "Aigbogun Alamba Daudu",
    "Murtala Afeez Bendu", 
    "Okorocha Calistus Ogbona",
    "Adewale Jimoh Akanbi",
    "Osazuwa Faith Etiyene",
    ];

    let ministeries = [
    "Internal Affairs",
    "Justice",
    "Defense",
    "Power & Steel",
    "Petroleum",
    ];

    let zones = [
    "South West",
    "North East",
    "South South",
    "South West",
    "South East",
    ];

    // Create output file
    let mut file = File::create("merged_ministeries.txt")
        .expect("Could not create file");

    // Print and write header
    println!("Merged EFCC Dataset");
    println!("S/N|    Name    |    Ministry    |    Geopolitical Zone\n");

    file.write_all(b"Merged EFCC Dataset\n")
        .expect("Failed to write header");
    file.write_all(b"S/N    |    Name    |    Ministry    |    Geopolitical Zone\n")
        .expect("Failed to write header");

    // Merge by index 
    for i in 0..names.len() {
        let line = format!(
            "{} | {} | {} | {}\n",
            i + 1,
            names[i],
            ministeries[i],
            zones[i],
        );

        // Display 
        print!("{}", line);

        // Write to file 
        file.write_all(line.as_bytes())
            .expect("Failed to write line");

    }
    println!("\nData saved to merged_ministeries.txt");
}