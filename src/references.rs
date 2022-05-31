
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
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
}
