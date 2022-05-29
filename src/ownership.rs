struct Foo {
    x: i32,
}

struct Bar {
    foo: Foo,
}

pub fn main() {
//     // We instantiate structs and bind to variables
//     // to create memory resources
//     let foo_a = Foo { x: 42 };
//     // foo is the owner
//     let foo_b = Foo { x: 13 };

//     println!("{}", foo_a.x);

//     println!("{}", foo_b.x);
//     // foo_b is dropped here 
//     // foo_a is dropped here
// }
    // Heirarchical ownership
    let bar = Bar { foo: Foo { x: 42 } };
    println!("{}", bar.foo.x);
    // bar is dropped first
    // then bar.foo is dropped
}
