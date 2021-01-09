#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect1 = (30, 50);

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect3 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect4 = Rectangle {
        width: 40,
        height: 60,
    };

    println!("The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    println!("The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    println!("The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );

    println!("The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    println!("rect2 is {:#?}", rect2);

    println!("Can rect 2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect 2 hold rect4? {}", rect2.can_hold(&rect4));

    let square1 = Rectangle::square(32);
    println!("{:#?}", square1);

}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
