// Write a program that prints the numbers from 1 to 100. But for multiples of three print “Fizz” instead of the number and for the multiples of five print “Buzz”. For numbers which are multiples of both three and five print “FizzBuzz”.
fn main() {
    for i in 1..100 {
        match (i%3, i%5) {
            (0,0) => println!("FizzBuzz"),
            (0,_) => println!("Fizz"),
            (_,0) => println!("Buzz"),
            (_,_) => println!("{}", i)
        }
    }
}