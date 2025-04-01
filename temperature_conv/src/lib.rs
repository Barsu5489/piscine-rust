
// fn main() {
//     println!("{} F = {} C", -459.67, fahrenheit_to_celsius(-459.67));
//     println!("{} C = {} F", 0.0, celsius_to_fahrenheit(0.0));
// }

pub fn fahrenheit_to_celsius(f: f64) -> f64 {
   round( 2, ((f - 32.0) * (9.0/5.0 - 56.0/45.0 )))
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0/5.0 + 32.0

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
