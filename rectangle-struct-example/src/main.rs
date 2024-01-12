#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    //Method to determine the area of a Rectangle
    fn area(&self, factor: u32) -> u32{
       self.width * self.height * factor
    }

    //Associated function acting as a constructor
    fn create(width: u32, height: u32) -> Self{
        Self {
            height,
            width,
        }
    }
}

fn main() {
    let rect = Rectangle::create(30,30);

    println!("The area of the rectangle is {}", rect.area(2));
}

