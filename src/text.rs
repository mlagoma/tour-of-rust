use indoc::indoc;

fn say_it_loud(msg:&str){
    println!("{}!!!",msg.to_string().to_uppercase());
}

fn parse_int(int: i32) -> Result<(), std::num::ParseIntError> {
    // let a = 42;
    // let a = "not a number";
    let a_string = int.to_string();
    let b = a_string.parse::<i32>()?;
    println!("{} {}", a_string, b);
    Ok(())
}

fn parse_int_from_string(str:&str) -> Result<(), std::num::ParseIntError> {
    // let a = 42;
    // let a = "not a number";
    let a_string = str.to_string();
    let b = a_string.parse::<i32>()?;
    println!("{} {}", a_string, b);
    Ok(())
}


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
    println!("find Option<usize> of 'not exists': {:?}\n", a.find("not exists"));

    // collect the characters as a vector of char
    let chars = "hi 🦀".chars().collect::<Vec<char>>();
    println!("{}", chars.len()); // should be 4
    // since chars are 4 bytes we can convert to u32
    println!("{}", chars[3] as u32);
    println!("{}\n", chars[3]);

    let mut helloworld = String::from("hello");
    helloworld.push_str(" world");
    helloworld = helloworld + "!";
    helloworld = helloworld.replace("world", "rust");
    println!("{}\n", helloworld);

    // say_it_loud can borrow &'static str as a &str
    say_it_loud("hello");
    // say_it_loud can also borrow String as a &str
    say_it_loud(&String::from("goodbye"));

    let helloworld = ["hello", " ", "world", "!"].concat();
    let abc = ["a", "b", "c"].join(",");
    println!("{}", helloworld);
    println!("{}\n",abc);

    let a = 42;
    let f = format!("secret to life: {}",a);
    println!("{}\n",f);

    let res = parse_int(42);
    println!("{:?}",res);
    let res = parse_int_from_string("42");
    println!("{:?}",res);
    let res = parse_int_from_string("NaN");
    println!("{:?}",res);
}