use std::io;

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // 配列の何番目の要素にアクセスするか指定してください
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("エラーなんだが");

    let index: usize = index.trim().parse().expect("ダメだった");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        // {}番目の要素の値は{}です
        index,
        element
    );
}
