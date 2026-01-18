fn apply<F>(f: F) where
    F: Fn() {
        f()
}

fn main() {
    let x = 7;

    // Capture x into anonymous type and implment `Fn` for it. Store it in `print`
    let print = || println!("{}", x);
    apply(print);
}