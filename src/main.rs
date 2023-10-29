use std::{env, fs, io, os::unix::prelude::MetadataExt};

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
        Ok(_) => println!("Searching for: {}", input_value),
        Err(_) => println!("Error!"),
    }

    let dir = fs::read_dir(current_dir_name.unwrap())?;

    // println!("dir {:?}", dir.unwrap());

    for el in dir {
        let el = el?;
        let path = el.path();

        // let file_metadata = fs::metadata(&path)?;
        // println!("from file_name : {}", file_metadata.is_file());
        if path.is_file() {
            let file = fs::metadata(&path)?;
            println!("this is a file: name ->{:?} type -->{:?}  size ->{:?}", path.file_name().unwrap(), path.extension(), file.size());
        }
        if path.is_dir() {
            println!("from is_dir: {:?}", path);
        }
    }

    // let metadata = fs::metadata(&input_value.trim().to_string());

    // match metadata {
    //     Ok(value) => println!("from meta {:?}", value.is_dir()),
    //     Err(e) => println!("Error from metadata: {}", e),
    // }

    Ok(())
}
