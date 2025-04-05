use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    if list.is_empty() {
        return 0.0;
    }
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    if list.is_empty() {
        return 0;
    }

    let mut sorted = list.to_vec();
    sorted.sort_unstable();

    let len = sorted.len();
    if len % 2 == 1 {
        sorted[len / 2]
    } else {
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut freq_map = HashMap::new();
    let mut max_freq = 0;
    let mut mode_val = list[0];

    for &num in list {
        let count = freq_map.entry(num).or_insert(0);
        *count += 1;

        if *count > max_freq {
            max_freq = *count;
            mode_val = num;
        }
    }

    mode_val
}