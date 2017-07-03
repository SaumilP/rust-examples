// An attribute to hide warnings for unused code.
#![allow(dead_code)]

enum Person {
    // unit-like
    Engineer,
    Scientist,

    // like tuple constructs
    Height(i32),
    Weight(i32),

    // or like strcutures
    Info { name: String, height: i32 }
}

fn inspect(p: Person) {
    // Usage of an `enum` must cover all cases
    match p {
        Person::Engineer => println!("Is an engineer !"),
        Person::Scientist => println!("Is a scientist !"),
        Person::Height(i) => println!("Has a height of {}.", i),
        Person::Weight(i) => println!("Has a weight of {}.", i),
        // Deconstruct `Info` into `name` and `height`
        Person::Info { name, height} => {
            println!("{} is {} tall !", name, height);
        },
    }
}

fn main() {
    let person = Person::Height(18);
    let joe = Person::Weight(50);
    let dave = Person::Info { name: "Dave".to_owned(), height: 72 };
    let rebecca = Person::Scientist;
    let jane = Person::Engineer;

    inspect(person);
    inspect(joe);
    inspect(dave);
    inspect(rebecca);
    inspect(jane);
}