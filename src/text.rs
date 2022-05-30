pub fn main() {
    let a: &'static str = "hi 🦀";
    println!("{} {}", a, a.len());

    let a: &'static str = "Ferris says:\t\"hello\"";
    println!("{}",a);
}
