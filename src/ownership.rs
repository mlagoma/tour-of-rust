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

fn remove_mutable_ownership(mut f: Foo) {
    //let y = f.x;
    println!("{}", f.x);
    f.x = 0;
    println!("{}", f.x);
    // f is dropped here
}

fn remove_borrowed_ownership(f: &mut Foo) {
    //let y = f.x;
    println!("{} (remove_borrowed_ownership before modify)", f.x);
    f.x += 1;
    println!("{} (remove_borrowed_ownership after modify)", f.x);
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

//     // Return ownership
//     let foo = return_ownership();
//     // foo becomes the owner
//     println!("{}", foo.x);
//     // foo is dropped because of end of function scope
// }

//     // Borrowing ownership (references)
//     let foo = Foo { x: 42 };
//     let f = &foo;
//     println!("{}", f.x);
//     // f is dropped here
//     // foo is dropped here
// }

//     // Borrowing Mutable Ownership with References
//     let mut foo = Foo { x: 42 };
//     let f = &mut foo;

//     // FAILURE: remove_ownership(foo) would fail because
//     // foo cannot be moved while mutably borrowed
//     // remove_ownership(foo);
//     // println!("{}", f.x);
//     // println!("{}", foo.x);

//     // FAILURE: foo.x = 13; would fail here because
//     // foo is not modifiable while mutably borrowed
//     // foo.x = 13;
//     // println!("{}", foo.x);
//     // println!("{}", f.x);

//     f.x = 13;
//     // f is dropped here because it's no longer used after this point
//     println!("{}", f.x);

//     println!("{}", foo.x);

//     // this works now because all mutable references were dropped
//     foo.x = 7;
//     println!("{}", foo.x);
//     // println!("{}", f.x);

//     // move foo's ownership to a function
//     remove_ownership(foo);
//     // remove_mutable_ownership(foo);
//     // println!("{}", foo.x);
// }

//     // Dereferencing
//     let mut foo = 42;
//     let f = &mut foo;
//     let bar = *f; // get a copy of the owner's value
//     *f = 13;      // set the reference's owner's value
//     println!("{}", bar);
//     println!("{}", foo);
// }

    // Passing Around Borrowed Data
    let mut foo = Foo { x: 42 };
    remove_borrowed_ownership(&mut foo);
    // println!("{}", f.x);
    // because all mutable references are dropped within
    // the function remove_ownership, we can create another.
    remove_borrowed_ownership(&mut foo);
    println!("{} (mutable [foo] after remove ownership)", foo.x);

    let f = &mut foo;
    println!("{} (mutable reference [f] before remove ownership)", f.x);
    remove_borrowed_ownership(f);
    // remove_ownership(foo);
    println!("{} (mutable reference [f] after remove ownership)", f.x);
    remove_borrowed_ownership(f);
    println!("{} (mutable [foo] after remove ownership)", foo.x);
    let f = &mut foo;
    println!("{}", f.x);
    // foo is dropped here
}
