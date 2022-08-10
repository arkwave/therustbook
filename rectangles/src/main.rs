#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32
}


fn main() {

    let rect = Rectangle {
        width: 30, 
        height: 50
    };
    
    let rect2 = Rectangle {
        width: 20, 
        height: 40
    };
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    println!("R1 can hold R2: {}", rect.can_hold(&rect2));
    
    let square = Rectangle::square(20);
    let mut mut_square = Rectangle::square(30);


    mut_square.width = 35; 

    dbg!(square);
    dbg!(mut_square);

}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height 
        
    }
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width:size,  height: size }
    }
}

//fn area(rectangle: &Rectangle) -> u32 {
//  rectangle.height * rectangle.width 
//}
