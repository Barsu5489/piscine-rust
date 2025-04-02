#[derive(PartialEq, Eq, Debug)]
pub struct  Matrix ( 
    pub (i32, i32),
    pub (i32, i32),
);
pub fn transpose(m: Matrix) -> Matrix {
    let (a, b) = (m.0, m.1); // Unwrap the two tuples
    Matrix(
        (a.0, b.0),  // First column becomes the first row
        (a.1, b.1),  // Second column becomes the second row
    )
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
