// 構造体を定義し、インスタンス化する

fn main() {
    let lect = (30, 50);

    println!(
        // 長方形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels.",
        area(lect)
    );
}

fn area(dimentions: (u32, u32)) -> u32 {
    dimentions.0 * dimentions.1
}