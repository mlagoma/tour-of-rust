struct Foo<'a> {
    i:&'a i32
}

pub fn main() {
    let x = 42;
    let foo = Foo {
        i: &x
    };
    println!("{}",foo.i);
}
