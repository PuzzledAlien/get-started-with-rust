// 经典结构
struct Person {
    name: String,
    age: u8,
    like_oranges: bool,
}

// 元组结构
struct Point2D(u32, u32);

// 经典结构
struct Unit;

fn main() {
    let person = Person {
        name : String::from("Adam"),
        like_oranges : true,
        age : 25
    };

    let orange = Point2D(0, 0);

    let unit = Unit;
}
