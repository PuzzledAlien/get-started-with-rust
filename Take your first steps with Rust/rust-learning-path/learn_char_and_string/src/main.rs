fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c is {}, z is {}, heart_eyed_cat is {}.", c, z, heart_eyed_cat);

    let mut hello = String::from("Hello, ");
    hello.push('w');
    hello.push_str("orld!");
    println!("{}", hello);

    hello = String::new();
    println!("hello:{}", hello);
}
