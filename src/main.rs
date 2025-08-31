fn main() {
    println!("Hello, world!");
    let x = Some(&1);
    match x {
        Some(i) if *i > 1 => {
            print!("do something")
        },
        _ => {
            print!("stil ok")
        }
    }
}
