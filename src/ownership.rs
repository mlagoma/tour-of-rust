struct Foo {
    x: i32,
}

struct Bar {
    foo: Foo,
}

fn remove_ownership(f: Foo) {
    println!("{}", f.x);
    // f is dropped here
}

fn return_ownership() -> Foo {
    Foo { x: 42 }
    // ownership is moved out
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
//     // Heirarchical ownership
//     let bar = Bar { foo: Foo { x: 42 } };
//     println!("{}", bar.foo.x);
//     // bar is dropped first
//     // then bar.foo is dropped
// }

//     // Remove ownership
//     let foo = Foo { x: 42 };
//     // foo is moved to do_something
//     println!("{}", foo.x);
//     remove_ownership(foo);
//     // foo can no longer be used
//     // println!("{}", foo.x);
// }

    // Return ownership
    let foo = return_ownership();
    // foo becomes the owner
    println!("{}", foo.x);
    // foo is dropped because of end of function scope
}
