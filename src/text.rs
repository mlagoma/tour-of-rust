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

    let a = "hi 🦀";
    let first_word = &a[0..2];
    let second_word = &a[3..7];
    // let half_crab = &a[3..5]; // FAILS
    // Rust does not accept slices of invalid unicode characters
    println!("{} {}", first_word, second_word);
    println!("length: {}", a.len());
    println!("find Option<usize> of ' ': {:?}", a.find(" "));
    println!("find Option<usize> of 'not exists': {:?}", a.find("not exists"));
}