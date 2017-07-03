// Time to create library , lets see how this can be created
//
// To create library :
// cmd> rustc --crate-type=lib rary.rs
pub fn public_funciton() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    print!("called rary's `private_function()`");
}

pub fn indirect_access() {
    println!("called rary's `indirect_access()`, that\n>");
    
    private_function();
}