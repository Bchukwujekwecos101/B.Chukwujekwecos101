use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::create("students.txt")
        .expect("Could not create file");

    //Array of student records 
    let students = [
        "Oluchi Mordi       ACC10211111      Accounting       100",
        "Adams Aliyu        ECO10110101      Economics        200",
        "Shania Bolade      CSC10238828      Computer         200",
        "Adekunle Gold      EEE11020202      Electrical       100",
        "Blanca Edemoh      MEE10202001      Mechanical       100",
        ];

        // Print header to screen
        println!("PAU SMIS");
        println!("Student Name   |   Matric Number   |   Department   |   Level\n");

        // Write header to file 
        file.write_all(b"PAU SMIS\n").expect("Failed to write");
        file.write_all(b"Student Name | Matric Number | Department | Level\n")
            .expect("Failed to write");

        // Display and write each student
        for student in students {
            println!("{}", student);
            file.write_all(student.as_bytes()).expect("Failed to write");
            file.write_all(b"\n").expect("Failed to write");
        }

        println!("\nSaved to students.txt!");
}