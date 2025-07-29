fn next_code(code: String) -> String {
    let mut chars: Vec<char> = code.chars().rev().collect(); // Reverse for easier carry
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == 'Z' {
            chars[i] = 'A';
            i += 1; // carry to next digit
        } else {
            chars[i] = ((chars[i] as u8) + 1) as char;
            return chars.into_iter().rev().collect(); // done
        }
    }

    // All characters were 'Z' (e.g., Z → AA, ZZ → AAA)
    chars.push('A');
    chars.into_iter().rev().collect()
}