fn main() {
    fn function (i : i32) -> i32 { i + 1 };

    // annotation is identical to function annotation but is optional as
    // `{}` wrapped body. These functions are annonymous functions
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i| i + 1;
    let closure_call_function = |i| function(i);

    let i = 1;

    // Lets call function and closures
    println!("function: {}", function(i));
    println!("annotated closure: {}", closure_annotated(i));
    println!("inferred closure: {}", closure_inferred(i));
    println!("inferred closure + function : {}", closure_call_function(i));

    // closure with no arguments
    let one = || 1;
    println!("closure returning one: {}", one());
    
    // you can capture variables from enclosing environment; something which is impossible
    // with functions
    let prof_x = "Charles Xavier";

    // closure with no arguent
    let print = || println!("Professor X's name is : {}", prof_x);

    // call the closure
    print();
}