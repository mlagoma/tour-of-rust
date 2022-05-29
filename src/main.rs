#![allow(dead_code)] // this line prevents compiler warnings

mod struct_enums;
mod option;
mod basic_result;
mod result;

// fn main() {
// 	// struct_enums::main();
// 	// option::main();
// 	basic_result::main();
// }

fn main() -> Result<(), String> {
	// return result::main();
	result::main()
}
