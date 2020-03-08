struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area (&self) -> u32 {
        &self.width * &self.height
    }
}

fn main() {
    
    let width = 30;
    let height = 50;

    let rect1 = Rectangle {
        width: 10, 
        height: 20
    };

    //println!("Are is {}", area(&rect1));
    println!("Are is {}", rect1.area());
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}