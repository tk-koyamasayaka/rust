// 構造体を定義し、インスタンス化する

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        // 長方形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}


// TODO タプルでリファクタリングする