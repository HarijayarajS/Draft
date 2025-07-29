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


fn next_custom_code(code: &str, alphabet: &[char]) -> String {
    let base = alphabet.len();
    let mut chars: Vec<char> = code.chars().rev().collect();
    let mut i = 0;
    let mut carry = true;

    while i < chars.len() {
        if carry {
            let pos = alphabet.iter().position(|&c| c == chars[i])
                .expect("Invalid character in input code");

            let next_pos = (pos + 1) % base;
            chars[i] = alphabet[next_pos];
            carry = next_pos == 0;
            i += 1;
        } else {
            break;
        }
    }

    if carry {
        chars.push(alphabet[0]);
    }

    chars.into_iter().rev().collect()
}