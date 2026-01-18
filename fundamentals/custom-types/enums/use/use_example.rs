// An attribute to hide warnings for unused code
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Explcitly `use` each name so they are available without manually scoping.
    use Status::{Poor, Rich};

    // Automatically `use` each name inside `Work`
    use Work::*;

    // Equivalent to `Status::Poor`
    let status = Poor;
    // Equivalent to `Work::Civilian`
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The rich have no money ..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight !"),
    }
}