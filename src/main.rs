#![allow(dead_code)] // this line prevents compiler warnings
#[allow(unused_imports)]
use crate::objects::NoiseMaker;
use core::fmt::Display;
use std::error::Error;

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
mod references;

// fn main() {
//  // struct_enums::main();
//  // option::main();
//  // basic_result::main();
//  vectors::main();
// }


struct FailablePie;

#[derive(Debug)]
struct NotFreshError;

impl Display for NotFreshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "This pie is not fresh!")
    }
}

impl Error for NotFreshError {}

impl FailablePie {
    pub fn eat(&self) -> Result<(), Box<dyn Error>> {
        Err(Box::new(NotFreshError))
    }
}

fn main() -> Result<(), String> {
    // return result::main();
    // result::main()
    // vectors::main();
    // ownership::main();
    // static_variables::main();
    // struct_lifetimes::main();
    // text::main();
    // let creature = objects::main();
    // // println!("{} goes {}", creature.name, creature.noise);
    // println!("{} goes {}", creature.name, creature.get_sound());
    // print!("{} goes ", creature.name);
    // creature.make_noise();
    // // creature.make_alot_of_noise();
    references::main();
    // references::failable_main();
    Ok(())
}

// // Doesn't report result
// fn failable_dereference() -> Result<(), Box<dyn Error>> {
//     // references::failable_main()
//     references::failable_main()?;
//     Ok(())
//     // let res = references::failable_main();
//     // println!("{:?}",res);
//     // res
//     // let heap_pie = references::failable_main();
//     // heap_pie.eat()?;
//     // Ok(())
// }

// // Result works
// fn main() -> Result<(), Box<dyn Error>> {
//     let heap_pie = Box::new(FailablePie);
//     heap_pie.eat()?;
//     Ok(())
// }