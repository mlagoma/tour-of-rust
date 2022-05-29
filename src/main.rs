#![allow(dead_code)] // this line prevents compiler warnings

mod struct_enums;
mod option;
mod basic_result;
mod result;
mod vectors;

// fn main() {
// 	// struct_enums::main();
// 	// option::main();
// 	// basic_result::main();
// 	vectors::main();
// }

fn main() -> Result<(), String> {
	// return result::main();
	// result::main()
	vectors::main();
	Ok(())
}
