pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let vec = s
        .split_whitespace()
        .filter_map(|word| {
            if let Some(stripped) = word.strip_suffix('k') {
                stripped.parse::<f32>().ok().map(|n| (n * 1000.0) as u32)
            } else {
                word.parse::<f32>().ok().map(|n| n as u32)
            }
        })
        .collect::<Vec<u32>>();

    Box::new(vec)
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a 
}