use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("drinks.txt")
        .expect("Failed to create file");

    let lager = "33 Export\nDesperados\nGoldberg\nGulder\nHeineken\nStar";
    let stout = "Legend\nTurbo King\nWilliams";
    let non_alcoholic = "Maltina\nAmstel Maltaa\nMalta Gold\nFayrouz";

    file.write_all(b"Lager:\n")
        .expect("Failed to write Lager header");
    file.write_all(lager.as_bytes())
        .expect("Failed to write Lager list");

    file.write_all(b"\n\nStout:\n")
        .expect("Failed to write Stout header");
    file.write_all(stout.as_bytes())
        .expect("Failed to write Stout list");

    file.write_all(b"\n\nNon_alcoholic:\n")
        .expect("Failed to write Non_alcoholic header");
    file.write_all(non_alcoholic.as_bytes())
        .expect("Failed to write Non_alcoholic list");

    println!("drinks.txt created successfully!");
}