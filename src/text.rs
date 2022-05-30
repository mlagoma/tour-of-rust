use indoc::indoc;

pub fn main() {
    let a: &'static str = "hi 🦀";
    println!("{} {}\n", a, a.len());

    let a: &'static str = "Ferris says:\t\"hello\"";
    println!("{}\n",a);

    let haiku: &'static str = indoc!("
        I write, erase, rewrite
        Erase again, and then
        A poppy blooms.
        - Katsushika Hokusai");
    println!("{}\n", haiku);

    println!("hello \
    world"); // notice that the spacing before w is ignored

    let a: &'static str = indoc!(r#"
        <div class="advice">
            Raw strings are useful for some situations.
        </div>
        "#);
    println!("{}\n", a);

    let hello_html = include_str!("hello.html");
    println!("{}\n", hello_html);
}
