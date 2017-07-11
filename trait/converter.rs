// Sample program converts from one unit to another
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeter(&self) -> Centimeters {
        // borrow self value into internal scoped value
        let &Inches(inches) = self;

        // returns constructed `Centimeters` from borrowed value after applying formulae
        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32);

fn main() {
    // `Second` has no implementation and so it can not be printed using macro `{:?}`
    let _one_second = Seconds(1);
    //println!("1 second = {:?}", _one_second);
    // ^ above statement fails until `Display::fmt` is implemented

    let foot = Inches(12);
    println!("1 foot = {:?}", foot);

    let meter = Centimeters(100.0);
    let comparison =
        if foot.to_centimeter() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("1 foot is {} than 1 meter.", comparison);
}
