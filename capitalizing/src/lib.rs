/// Capitalizes the first letter of the string
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// Capitalizes the first letter of every word in the string
pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize = true;

    for ch in input.chars() {
        if ch.is_alphabetic() && capitalize {
            result.push_str(&ch.to_uppercase().to_string());
            capitalize = false;
        } else {
            result.push(ch);
            if ch.is_whitespace() {
                capitalize = true;
            }
        }
    }

    result
}

/// Changes lowercase to uppercase and uppercase to lowercase
pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
}