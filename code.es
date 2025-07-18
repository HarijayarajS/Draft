use std::collections::HashSet;

fn get_next_code(allowed: &[&str], used: &[&str]) -> Option<String> {
    let used_set: HashSet<String> = used.iter().map(|s| s.to_string()).collect();

    // Step 1: Check all 1-part codes
    for &code in allowed {
        if !used_set.contains(code) {
            return Some(code.to_string());
        }
    }

    // Step 2: Check all 2-part codes
    for &prefix in allowed {
        for &suffix in allowed {
            let combined = format!("{}{}", prefix, suffix);
            if !used_set.contains(&combined) {
                return Some(combined);
            }
        }
    }

    // All used
    None
}

fn main() {
    let allowed = vec![
        "A", "B", "C", "D", "E", "F", "G", "H", "I",
        "יאי", "ניי", "2", "יצי", "ישי", "יטי", "יזי",
        "P", "0", "R", "S", "אי", "L"
    ];

    // Simulate used codes (1-char and 2-char)
    let mut used: Vec<String> = allowed.iter().map(|s| s.to_string()).collect();
    for a in &allowed {
        for b in &allowed {
            used.push(format!("{}{}", a, b));
        }
    }

    // Remove one to simulate it's available
    used.retain(|c| c != "AA");

    let used_refs: Vec<&str> = used.iter().map(|s| s.as_str()).collect();

    match get_next_code(&allowed, &used_refs) {
        Some(code) => println!("Next available code: {}", code),
        None => println!("All codes are used."),
    }
}