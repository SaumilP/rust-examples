// Sample program demonstrates overloadable operators.

// Operators which would be overloaded in this sample program.
use std::ops::{Add, Sub};

#[derive(Debug)]
struct Point {
    x : i32,
    y : i32,
}

// implementation for overloadable operator `std::ops::Add`. This trait is used to specify
// functionality of `+`. Below implementation overrides default behaviour
impl Add for Point {
    // resultant type definition
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// implementation for overloadable operator `std::ops::Sub` for `-`  (Same as above).
impl Sub for Point {
    // same as above - type definition
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn main() {
    println!("{:?}", Point { x: 1, y: 0} + Point { x:2, y: 3});
    println!("{:?}", Point { x: 3, y: 4} - Point { x:2, y: 1});
    println!("{:?}", Point { x: 3, y: 0} - Point { x:2, y: 3});
}
