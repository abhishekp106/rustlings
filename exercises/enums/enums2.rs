// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!


struct Point {
    x: u8,
    y: u8
}

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move((u8, u8)),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move ( (10, 30) ),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
