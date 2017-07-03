// Sample program without using structure ( before )
fn main() {
    let width = 30;
    let length = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area(length, width)
    );
}

fn area(length: u32, width: u32) -> u32 {
    length * width
}