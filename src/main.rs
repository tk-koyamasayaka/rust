fn main() {
    let a = no_dangle();
    
    println!("{}", a)
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}