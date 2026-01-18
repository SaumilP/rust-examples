fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = {};

    let copied_int = an_integer;
    println!("An integer: {:?}", copied_int);
    println!("A Boolean: {:?}", a_boolean);
    println!("Meet the Unit value: {:?}", unit);

    // warning is suppresed when variable starts with `_`.
    let _unused_var = 3u32;
    let _noisy_unused_var = 2u32;
}