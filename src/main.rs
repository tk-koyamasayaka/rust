// 構造体を定義し、インスタンス化する
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let lect = Rectangle { width: 30, height: 50};

    println!(
        // 長方形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels.",
        area(&lect)
    );
}

// 構造体の所有権を奪うよりも借用する必要があります。こうすることでmainは所有権を保って、
// rect1を使用し続けることができ、そのために関数シグニチャと関数呼び出し時に&を使っている
fn area(dimentions: &Rectangle) -> u32 {
    dimentions.width * dimentions.height
}