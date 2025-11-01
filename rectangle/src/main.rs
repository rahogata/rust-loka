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

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle with variables is {} square pixels.",
        area(width1, height1)
    );
    let rect1 = (40, 50);
    println!(
        "The are of the rectangle with tuple is {} square pixels.",
        area_tuple(rect1)
    );
    let rect = Rectangle {
        width: dbg!(59 * 4),
        height: 48,
    };
    println!(
        "The area of the rectangle with struct is {} square pixels.",
        area_rect(&rect)
    );
    println!("rect is {rect:?}");
    println!("rect is {rect:#?}");
    dbg!(&rect);
    println!(
        "The are of the rectangle with method is {} square pixels.",
        rect.area()
    );
    let rectl1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rectl2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rectl3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rectl1 hold rectl2? {}", rectl1.can_hold(&rectl2));
    println!("Can rectl1 hold rectl2? {}", rectl1.can_hold(&rectl3));
    let sq = Rectangle::square(34);
    println!("Created square {sq:?}");

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_rect(r: &Rectangle) -> u32 {
    r.width * r.height
}