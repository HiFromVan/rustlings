#[derive(Debug)]
enum Message {
    // 1. 无数据关联（类似传统枚举）
    Quit,

    // 2. 关联一个匿名的字符串
    Echo(String),

    // 3. 关联一个匿名元组 (x, y)
    Move { x: i32, y: i32 }, // 也可以用 Move(i32, i32)，但通常坐标用结构体形式更清晰

    // 4. 关联一个匿名结构体
    Resize { width: u32, height: u32 },

    // 5. 关联多个数值（RGB 颜色）
    ChangeColor(i32, i32, i32),
}

fn main() {
    // 注意：在 main 中直接打印枚举定义需要手动补充实例化，
    // 原题目中 main 里的写法其实是在调用这些变体。
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("Hello")));
    println!("{:?}", Message::Move { x: 10, y: 20 });
    println!("{:?}", Message::Resize { width: 100, height: 200 });
    println!("{:?}", Message::ChangeColor(255, 0, 0));
}