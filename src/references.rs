use std::ops::Deref;
use std::alloc::{alloc, Layout};

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

struct Pie {
    secret_recipe: usize,
}
impl Pie {
    fn new() -> Self {
        // let's ask for 4 bytes
        let layout = Layout::from_size_align(4, 1).unwrap();

        unsafe {
            // allocate and save the memory location as a number
            let ptr = alloc(layout) as *mut u8;
            // use pointer math and write a few 
            // u8 values to memory
            ptr.write(86);
            ptr.add(1).write(14);
            ptr.add(2).write(73);
            ptr.add(3).write(64);

            Pie { secret_recipe: ptr as usize }
        }
    }
}
impl Deref for Pie {
    type Target = f32;
    fn deref(&self) -> &f32 {
        // interpret secret_recipe pointer as a f32 raw pointer
        let pointer = self.secret_recipe as *const f32;
        // dereference it into a return value &f32
        unsafe { &*pointer }
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

    let p = Pie::new();
    println!("{}", p.secret_recipe);
    // "make a pie" by dereferencing our 
    // Pie struct smart pointer
    println!("{:?}", *p);
}
