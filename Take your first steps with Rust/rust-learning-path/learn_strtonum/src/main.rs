fn main() {
    let number1: u32 = "42".parse().expect("Not a number!");
    println!("the number1 is {}.", number1);

    let number2 = "42".parse::<u32>().expect("Not a number!");
    println!("the number2 is {}.", number2);
}
