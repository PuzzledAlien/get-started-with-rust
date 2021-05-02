
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(KeyPress),
    Paste(String),
    Click(Click)
}

struct Click { 
    x: i64, 
    y: i64 
}

struct KeyPress(char);