use std::{env, fs, io, path::PathBuf};
use colored::*;

#[derive(Debug)]
enum Ford {
    File,
    Dir,
}

fn get_contents() -> Vec<PathBuf> {
    //get the contents of the desired directory
    fs::read_dir("./")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect::<Vec<_>>() // put this into a match for whether user included
}
fn main() {
    let content = get_contents();
    for path in content.iter() {
        match path.ends_with("/") {
            true => {
                let direc = path.to_str();
                println!("{}",direc.expect("REASON").red());
            }
            false => {
                let file = path.to_str();
                println!("{}",file.expect("REASON").blue())
            }
            
        }
    }
}
