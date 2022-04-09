use std::{env, process};

pub fn get_file_names_from_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Please specify a path to file");
        println!("usage: rttp <filename>");
        process::exit(0);
    }

    args[1..].to_vec()
}
