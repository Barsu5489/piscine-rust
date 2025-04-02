pub fn str_len(s: &str) -> usize {
return s.len()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_len_single_char() {
        let result = str_len("a"); // Test with a string of one character
        assert_eq!(result, 1); // The length should be 1
    }
    #[test]
    fn test_str_lenmult_char(){
        let result = str_len("hello");
        assert_eq!(result, 5)
    }
}
