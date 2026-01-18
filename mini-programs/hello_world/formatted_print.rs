fn main(){
    // basic println
    println!("{} days", 31);

    // various arguments
    println!("{0}, this is {1}, {1}, this is {0}", "Alice", "bob");

    // named arguments
    println!("{subject} {verb} {obj}",
             obj="the lazy dog",
             subject="quick brown fox",
             verb="jumps over");

    // special formatting
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // It will even check to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Create a structure which contains an `i32`. Name it `Structure`.
    #[derive(Debug)]
    struct Structure{
        a: i32
    }

    // However, custom types such as this structure require more complicated
    // handling. 
    let x = Structure{ a:3 };
    println!("This struct `{:?}` won't print...", x);
}
