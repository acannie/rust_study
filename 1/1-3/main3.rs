fn main() {
    // let objective: Option<i32> = Some(1);
    let objective: Option<i32> = Some(2);
    let objective: Option<i32> = None;
    match objective {
        Some(x) if x % 2 == 0 => println!("x is even."),
        Some(x) => println!("x is odd."),
        None => println!("x is None."),
    }
}
