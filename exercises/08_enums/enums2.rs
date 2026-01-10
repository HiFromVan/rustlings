#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    // 1. 无数据关联（类似传统枚举）
    Quit,

    // 2. 关联一个匿名的字符串
    Echo(String),

    // 3. 关联一个匿名元组 (x, y)
    Move (Point ), // 也可以用 Move(i32, i32)，但通常坐标用结构体形式更清晰

    // 4. 关联一个匿名结构体
    Resize { width: u32, height: u32 },

    // 5. 关联多个数值（RGB 颜色）
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
