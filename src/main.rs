use std::io;
use std::cmp::Ordering;
use rand::Rng; // 乱数生成のためのメソッドを定義

fn main() {
    println!("Guess the number!");

    /*
     * thread_rng -> Rngトレイトの中で定義されている
     * 1..101 -> 開始..終了(1 <= x < 101 の意味)
     * 1.=100 これでもいい
     */
    let secret_number = rand::thread_rng().gen_range(1..101); 

    println!("秘密の数字は次の通りだよ {} ", secret_number);

    println!("数字をに入力してね");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = guess.trim().parse()
    .expect("数字を入力してください");

    println!("あなたが決めた数字 {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}