// Link to `library`, import items under the `rary` module
extern crate rary;

fn main() {
    rary::public_funciton();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}