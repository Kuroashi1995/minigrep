use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    // let query = &args[1];
    let target = &args[2];

    println!("Reading file: {target}");

    let contents = fs::read_to_string(target).expect("Should've been able to read file content");

    println!("with content:\n{contents}"); //This macro does not take ownership, do all macros
                                           //behave like this?
}
