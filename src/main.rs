use std::{env, fs, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let mut input_value = String::new();
    let get_input = io::stdin().read_line(&mut input_value);

    match get_input {
        Ok(value) => println!("input value: {}", value), //give the length of the input
        Err(_) => println!("Error!"),
    }

    let metadata = fs::metadata(&input_value.trim());

    match metadata {
        Ok(value) => println!("from meta {:?}", value),
        Err(e) => println!("Error: {}", e),
    }

    //get current directory
    let current_dir = env::current_dir().unwrap();
    let current_exc = env::current_exe();
    println!(
        "{:?}, {:?}",
        current_dir.components().next().unwrap().as_os_str(),
        current_exc
    );

    println!("{}", input_value);
    // println!("{}", metadata);

    Ok(())
}
