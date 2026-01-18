// simple macro for printing "Hello!"
macro_rules! say_hello {
    // `()` indicates that macro takes no argument.
    () => (
        println!("Hello!");
    )
}

fn main() {
    // this call will expand into `println!("Hello!");`
    say_hello!();
}
