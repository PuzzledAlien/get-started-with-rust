fn main() {
    let a_number = 10;
    println!("the number is {}.", a_number);
    a_number = 15; // error[E0384]: cannot assign twice to immutable variable `a_number`
    println!("and now the number is {}.", a_number);
}
