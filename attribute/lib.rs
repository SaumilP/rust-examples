// This example shows how to customize the library name
//
// cmd> rustc lib.rs

#![crate_type = "lib"]
// The library is named `rary`
#![crate_name = "rary"]

pub fn public_funciton() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_function() {
    println!("called rary's `indirect_function()`, that\n>");
    
    private_function();
}