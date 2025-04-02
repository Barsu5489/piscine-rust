pub fn insert(vec: &mut Vec<String>, val: String) {

return vec.push(val);

}

pub fn at_index(slice: &[String], index: usize) -> &str {
    let second = &slice[index];
    return second
}
fn main() {
    let mut groceries = vec![
        "yogurt".to_string(),
        "panettone".to_string(),
        "bread".to_string(),
        "cheese".to_string(),
    ];
    insert(&mut groceries, String::from("nuts"));
    println!("groceries = {:?}", &groceries);
     println!("groceries[1] = {:?}", at_index(&groceries, 1));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
