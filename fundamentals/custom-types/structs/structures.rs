use std::fmt::{self, Formatter, Display};

// A unit struct
struct Nil;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

// Implements Display for Rectangle type
impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.p1.x, self.p1.y, self.p2.x, self.p2.y)
    }
}

fn rect_area(rectangle: Rectangle) {
    // Deconstruct rectangle
    let Rectangle { p1: top_left_point, p2: bottom_right_point } = rectangle;

    // Deconstruct points
    let Point { x: top_left_x, y: top_right_y } = top_left_point;
    let Point { x: bottom_left_x, y: bottom_right_y } = bottom_right_point;

    // calculate area by determining width and height
    bottom_left_x - top_left_x * bottom_right_y - top_right_y;
}

fn square(point: Point, wd: f32) -> Rectangle {
    // Deconstruct provided point for using properties for constructing new Rectangle
    let Point { x: my_x, y: my_y } = point;

    // Construct new Rectangle and return it as response
    Rectangle {
        p1: point,
        p2: Point { x: my_x + wd, y : my_y + wd },
    }
}

fn main() {
    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4};

    println!("point coordinates: ({} {})", point.x, point.y);

    // Destructure the point using a `let` binding
    let Point { x: my_x, y: my_y } = point;
    println!("destructured points are {:?} and {:?}", my_x, my_y);

    let _rectangle = Rectangle {
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Initiaite a tuple struct
    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair( integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    
    // Activity:1 add `rect_area` which calculates the area of a rectangle
    println!("area of rectangle is {:?}", rect_area(_rectangle));

    // Activity:2 Add a function `square` which takes a `Point` and `f32` as 
    // arguments and returns a `Rectangle` with its lower left corner on the point, 
    // and a width and height corresponding to the `f32`.
    let new_point: Point = Point { x: 0.1, y: 0.6};
    println!("area of square is {}", square(new_point, 0.2));
}