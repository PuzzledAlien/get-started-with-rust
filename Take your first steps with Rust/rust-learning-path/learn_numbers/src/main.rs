fn main() {

    let x = 2.0; // f64, default type
    println!("x: {}", x);

    let y: f32 = 3.0; // f32, via type annotation
    println!("y: {}", y);

    println!("1 + 2 = {}", 1u32 + 2);

    println!("1 - 2 = {}", 1i32 - 2);
    // ^ Try changing `1i32` to `1u32` to see why the type is important
    // 1u32: error: this arithmetic operation will overflow (1u32会溢出)

    println!("9 / 2 = {}", 9u32 / 2);

    println!("9 / 2 = {}", 9.0 / 2.0);

    println!("3 * 6 = {}", 3 * 6);
}
