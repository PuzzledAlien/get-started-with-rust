fn main() {
    let number = "42".parse().expect("Not a number!"); // error[E0282]: type annotations needed
    println!("the number is {}.", number);
}
