// struct Rectangle{
//     width : u32,
//     length: u32
// }
// impl  Rectangle{
//     fn square(&self) -> bool{
//         self.width == self.length
//     }
// }

// fn main(){
//     let rect = Rectangle{width:5, length: 7};
//     println!("Rectangle is square {}", rect.square())
// }

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
	pub alias: String,
	pub brightness: u8,
}


impl Light {
    pub fn new(alias: &str) -> Self {
    Self { 
        alias: alias.to_string(),
         brightness: 0 }
    }
}

pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    if let Some(lgt) =  lights.iter_mut().find(|lgt|lgt.alias == alias){
        lgt.brightness = value;
    }
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
