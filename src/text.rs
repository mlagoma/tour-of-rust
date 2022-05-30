pub fn main() {
    let a: &'static str = "hi 🦀";
    println!("{} {}", a, a.len());

    let a: &'static str = "Ferris says:\t\"hello\"";
    println!("{}",a);

    let haiku: &'static str = "
        I write, erase, rewrite
        Erase again, and then
        A poppy blooms.
        - Katsushika Hokusai";
    println!("{}", haiku);
    
    
    println!("hello \
    world") // notice that the spacing before w is ignored
}
