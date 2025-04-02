pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut vex = Vec::new(); // Store results

    for ch in names {
        let words: Vec<&str> = ch.split_whitespace().collect(); // Split by spaces

        let mut initial_str = String::new(); // Store initials

        if let Some(first_word) = words.get(0) {
            if let Some(init_one) = first_word.chars().nth(0) {
                initial_str.push(init_one);
                initial_str.push('.'); // Add dot
                initial_str.push(' '); // Add space
            }
        }

        if let Some(second_word) = words.get(1) {
            if let Some(init_two) = second_word.chars().nth(0) {
                initial_str.push(init_two);
                initial_str.push('.'); // Add dot
            }
        }

        vex.push(initial_str); // Store in results vector
    }
    
    vex
}

