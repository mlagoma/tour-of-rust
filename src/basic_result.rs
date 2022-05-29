fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

pub fn main() {
    let result = do_something_that_might_fail(12);
    // let result = do_something_that_might_fail(42);

    // match lets us deconstruct Result elegantly and ensure we handle all cases!
    match result {
        Ok(v) => println!("found {}", v),
        Err(e) => println!("Error: {}",e),
    }
}
