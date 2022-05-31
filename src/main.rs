#![allow(dead_code)] // this line prevents compiler warnings

mod struct_enums;
mod option;
mod basic_result;
mod result;
mod vectors;
mod ownership;
mod static_variables;
mod struct_lifetimes;
mod text;
mod objects;

// fn main() {
// 	// struct_enums::main();
// 	// option::main();
// 	// basic_result::main();
// 	vectors::main();
// }

fn main() -> Result<(), String> {
	// return result::main();
	// result::main()
	// vectors::main();
	// ownership::main();
	// static_variables::main();
	// struct_lifetimes::main();
	// text::main();
	objects::main();
	Ok(())
}
