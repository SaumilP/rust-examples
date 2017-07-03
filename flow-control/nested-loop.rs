#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entering into the outer loop");

        'inner: loop {
            println!("Entering into the inner loop");

            // breaks inner loop
            //break;
            // TRY ^ uncommenting above break and see what is the effect of loops

            // breaks outer loop
            //break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}