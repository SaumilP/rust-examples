fn main() {
    let logical: bool = false;
    println!("logical := {}", logical);
    
    let a_float: f64 = 1.0;  // Regular annotation
    println!("a_float := {:?}", a_float);

    let an_integer   = 5i32; // Suffix annotation
    println!("an_integer := {}", an_integer);

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    println!("default_float := {:?}", default_float);

    let default_integer = 7;   // `i32`
    println!("default_integer := {}", default_integer);
}