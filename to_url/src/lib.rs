pub fn to_url(s: &str) -> String {
  
    let mut app = String::new();
    for char in s.chars(){
        if char == ' '{
            app += "%20";
        }
        else{
            app.push(char);
        }
    }
    return app;
}
fn main() {
    let s = "Hello, world!";
    println!("'{}' parsed as an URL becomes '{}'", s, to_url(s));
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
