#[derive(Debug)]
struct Rectangle {
    height : u32,
    width  : u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_fit(&self, rect2: &Rectangle) -> bool {
        self.height >= rect2.height && 
        self.width >= rect2.width
        
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    let rect1 = Rectangle{height:30, width:50};

    println!(
        "Area of rect 1 is {:#?}",
        rect1.can_fit(&Rectangle{height:40, width:10})
    );
}

