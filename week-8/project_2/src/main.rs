use std::cmp::Ordering;

// A compound data type: struct
#[derive(Debug)]
struct Developer {
    name: String,
    years_experience: u32,
}

fn main() {
    // Vector of developers
    let applicants = vec![
        Developer { name: String::from("Brian Chukwujekwe"), years_experience: 3 },
        Developer { name: String::from("Divine Ikubor"), years_experience: 7 },
        Developer { name: String::from("David Adeleke"), years_experience: 5 },
        Developer { name: String::from("Jamil Balogun"), years_experience: 9 },
        Developer { name: String::from("Sandra Kennedy"), years_experience: 4 },
    ];

    // Assume first applicant has highest experience initially
    let mut highest = &applicants[0];

    // Loop through the vector to find the highest years
    for dev in &applicants {
        if dev.years_experience > highest.years_experience {
            highest = dev;
        }
    }

    println!("=======================================");
    println!(" Developer with the Highest Experience ");
    println!("=======================================");
    println!("Name: {}", highest.name);
    println!("Years of Experience: {}", highest.years_experience);
}
