use std::{env, fs, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //get current directory name
    let current_dir = env::current_dir().unwrap();

    let current_dir_name = current_dir
        .components()
        .next()
        .unwrap()
        .as_os_str()
        .to_str();

    if let Some(i) = current_dir_name {
        println!("current directory: {:?}", i);
    } else {
        panic!("can't find the directory name")
    }

    //Get user input for the file name
    println!("Search for a file:");
    let mut input_value = String::new();
    let get_input = io::stdin().read_line(&mut input_value);

    //check if user input is ok
    match get_input {
        Ok(_) => println!("Searching for {}", input_value),
        Err(_) => println!("Error!"),
    }

    let metadata = fs::metadata(&input_value.trim());

    match metadata {
        Ok(value) => println!("from meta {:?}", value),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
