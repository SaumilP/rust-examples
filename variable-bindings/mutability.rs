fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // ok
    mutable_binding += 1;
    println!("After mutation : {}", mutable_binding);

    //Error if attempted to change immutable variable
    //_immutable_binding += 1;
}