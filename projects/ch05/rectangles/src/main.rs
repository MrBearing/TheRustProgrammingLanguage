fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


fn area_with_size(width: u32, height: u32) -> u32 {
    width * height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}


fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_size(width1, height1)
    );


    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_tuple(rect1)
    );

    let struct_rect = Rectangle{width: 30, height: 30};
    println!("rect1 is {:?}", struct_rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        struct_rect.area()
    );

}

