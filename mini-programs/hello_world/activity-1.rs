fn main(){
    let pi = 3.141592;
    let formatted_pi = format!("Pi: {:.*}", 3, pi);
    println!("{}", formatted_pi);
}
