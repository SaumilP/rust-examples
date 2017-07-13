fn main() {
    fizzbuzz_to(100);
}

// Simple function that returns a boolean value if given number can be divided
fn is_divided_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

// Returns a function
fn fizzbuzz(n: u32) -> () {
    if is_divided_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divided_by(n, 3) {
        println!("fizz");
    } else if is_divided_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n)
    }
}