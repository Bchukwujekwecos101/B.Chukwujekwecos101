use std::io;

// Structure to hold APS level info
struct ApsLevel {
    level: &'static str,
    office_admin: &'static str,
    academic: &'static str,
    lawyer: &'static str,
    teacher: &'static str,
}

fn main() {
    // Vector containing all APS levels
    let aps_table = vec![
        ApsLevel { level: "APS 1-2", office_admin: "Intern", academic: "-", lawyer: "Paralegal", teacher: "Placement" },
        ApsLevel { level: "APS 3-5", office_admin: "Administrator", academic: "Research Assistant", lawyer: "Junior Associate", teacher: "Classroom Teacher" },
        ApsLevel { level: "APS 5-8", office_admin: "Senior Administrator", academic: "PhD Candidate", lawyer: "Associate", teacher: "Snr Teacher" },
        ApsLevel { level: "EL1 8-10", office_admin: "Office Manager", academic: "Post-Doc Researcher", lawyer: "Senior Associate 1-2", teacher: "Leading Teacher" },
        ApsLevel { level: "EL2 10-13", office_admin: "Director", academic: "Senior Lecturer", lawyer: "Senior Associate 3-4", teacher: "Deputy Principal" },
        ApsLevel { level: "SES", office_admin: "CEO", academic: "Dean", lawyer: "Partner", teacher: "Principal" },
    ];

    // Input: Staff profession
    println!("Enter profession (office, academic, lawyer, teacher):");
    let mut profession = String::new();
    io::stdin().read_line(&mut profession).unwrap();
    let profession = profession.trim().to_lowercase();

    // Input: Years of experience
    println!("Enter years of experience:");
    let mut years = String::new();
    io::stdin().read_line(&mut years).unwrap();
    let years: i32 = years.trim().parse().unwrap();

    // Determine APS category by experience
    let index = if years <= 2 {
        0
    } else if years <= 5 {
        1
    } else if years <= 8 {
        2
    } else if years <= 10 {
        3
    } else if years <= 13 {
        4
    } else {
        5
    };

    let aps = &aps_table[index];

    // Output based on profession
    let title = match profession.as_str() {
        "office" => aps.office_admin,
        "academic" => aps.academic,
        "lawyer" => aps.lawyer,
        "teacher" => aps.teacher,
        _ => "Unknown profession",
    };

    println!("\n==============================");
    println!("Profession: {}", profession);
    println!("Years of Experience: {}", years);
    println!("APS Level: {}", aps.level);
    println!("Title: {}", title);
    println!("==============================");
}
