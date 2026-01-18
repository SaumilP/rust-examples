// Sample application to test out how threading can work

use std::thread;

fn main() {
    // Lets make a mutable vector as empty container
    let mut children = vec![];

    // Lets push different types of data into this container
    let numbers = [1usize, 2, 3, 4];
    let names = ["Tim", "Eston", "Aaron", "Ben", "Bob", "John", "James", "Johnny"];
    let odds = numbers.iter().map(|&x| x * 2 - 1 );

    for number in odds {
        children.push(thread::spawn(move || {
            println!("{} says hello from a lightweight thread!", names.get(number).unwrap());
        }));
    }

    for child in children {
        // Await for child specific thread to finish, and returns as result.
        let _ = child.join();
    }
}