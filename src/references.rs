use std::ops::Deref;

struct Foo {
    value: i32
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

struct TattleTell<T> {
    value: T,
}
impl<T> Deref for TattleTell<T> {
    type Target = T;
    fn deref(&self) -> &T {
        println!("{} was used!", std::any::type_name::<T>());
        &self.value
    }
}

pub fn main() {
    let a = 42;
    let memory_location = &a as *const i32 as usize;
    println!("Data is here {}", memory_location);
    // print_type_of(&memory_location);
    // print_type_of(&a);
    let a: i32 = 42;
    let ref_ref_ref_a: &&&i32 = &&&a;
    let ref_a: &i32 = **ref_ref_ref_a;
    let b: i32 = *ref_a;
    // println!("{}", ref_a);
    println!("{}", b);
    // print_type_of(&ref_ref_ref_a);
    // print_type_of(&ref_a);

    let f = Foo { value: 42 };
    let ref_ref_ref_f = &&&f;
    println!("{}", ref_ref_ref_f.value);

    let foo = TattleTell {
        value: "secret message",
    };
    // dereference occurs here immediately 
    // after foo is auto-referenced for the
    // function `len`
    println!("after deref: {}", foo.value);
    println!("{}", foo.len());
    println!("after deref: {}", foo.value);

    let a: [u8; 4] = [86, 14, 73, 64];
    // this is a raw pointer. Getting the memory address
    // of something as a number is totally safe
    println!("{}", a[0]);
    let pointer_a = &a as *const u8 as usize;
    println!("Data memory location: {}", pointer_a);
    // Turning our number into a raw pointer to a f32 is
    // also safe to do.
    let pointer_b = pointer_a as *const f32;
    let pointer_c = pointer_b as *const f32;
    println!("Data memory location: {}", pointer_b as usize);
    let b = unsafe {
        // This is unsafe because we are telling the compiler
        // to assume our pointer is a valid f32 and
        // dereference it's value into the variable b.
        // Rust has no way to verify this assumption is true.
        *pointer_c
    };
    println!("I swear this is a pie! {}", b);
}
