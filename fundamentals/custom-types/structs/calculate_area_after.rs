// Sample program using tuples and strcutures
#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    // Use of tuples ....
    let rect = (50, 30);

    println!(
        "[Tuple] The area of the rectangle is {} square pixels",
        area(rect)
    );

    // Use of structs ....
    let rect1 = Rectangle { length: 50, width: 30};
    println!(
        "[Struct] The area of the rectangle is {} square pixels",
        rect_area(&rect1)
    );
    println!("rect1 is {:?}", rect1);
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn rect_area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}