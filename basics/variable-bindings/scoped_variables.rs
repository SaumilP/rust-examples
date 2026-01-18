fn main() {
    let long_lived_binding = 2;

    // block shortens variable scope instead of main
    {
        let short_lived_bindings = 10;
        println!("inner short: {}", short_lived_bindings);

        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }

    println!("outer long: {}", long_lived_binding);

    // this binding also *shadow* the previous binding
    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}