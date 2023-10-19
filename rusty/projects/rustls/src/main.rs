use std::{env,fmt::{self, format}, fs, io, path::{Path,PathBuf}, vec, ops::Deref};
use colored::*;
use crossterm::cursor;
use termion::{cursor::DetectCursorPos, raw::IntoRawMode};

#[derive(Debug)]
enum Ford {
    File,
    Dir,
}

struct Item {
    ford: Ford,
    name: String,
}

fn get_termsize() -> (u16,u16) {
    termion::terminal_size().unwrap()
}

fn get_contents() -> Vec<PathBuf> {
    //get the contents of the desired directory
    fs::read_dir(".")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect::<Vec<_>>() // put this into a match for whether user included
}

// fn get_args
//

fn get_long<'a>(content: &Vec<PathBuf>) -> Option<PathBuf> {
    let pathb = PathBuf::from(" ");

    if content.len() == 0 {return None}
    else{
        let long = content.iter().fold(pathb, |acc,item| {
            if acc.as_os_str().len() > item.as_os_str().len() { 
                acc.to_path_buf()
            } else {
                item.to_path_buf()
            }

        });
        return Some(long);


    }
    // return long

}

fn itemprint(item: &Item, long: &usize) {
    match item.ford {
        Ford::File => print!("{:<}  ", item.name.red()),//,width = long),
        Ford::Dir => print!("{:<}  ", item.name.green()),//,width = long),
    }
}

fn sort(content: &Vec<PathBuf>) -> Vec<Item> {
    let mut all: Vec<Item> = Vec::new();
    for path in content.iter() {

        match Some(path.is_dir()) {
            Some(true) => {
                let Some(direc) = path.to_str() else { todo!()};
                all.push(Item { ford: Ford::Dir, name: direc[2..direc.len()].to_string()});

            },
            Some(false) => {
                let Some(file) = path.to_str() else { todo!()};
                all.push(Item { ford: Ford::File, name: file[2..file.len()].to_string()});

            },
            None => todo!()
        }
    }
    return all
}
fn checkspace(width: usize, length: &usize) {
    let Ok((x, y)): Result<(u16, u16), io::Error> = std::io::stdout().into_raw_mode().unwrap().cursor_pos() else { todo!() };
    if (width - x as usize) <= *length {
        print!("\n")
        // print!("\r");
    }
}


fn main() {
    let content = get_contents();
    let (width, _) = get_termsize();
    let binding = get_long(&content).unwrap();
    let longest = binding.to_str().expect("REASON");
    let long = longest.len();
    let all = sort(&content);
    println!("{} {}",width,long);
    for item in all.iter() {
        checkspace(width as usize, &item.name.len().into());
        itemprint(item, &long);
    }
    print!("\n")
}


            // true => {
            //     // direcs.push(direc);
            //     // print!(" {:} ",direc.expect("REASON").yellow());
            //     print!(" {:} ",direc);
            //     counter = counter + 1;
            // },
            // false => {
            //     let Some(file?) = path.to_str();
            //     // files.push(file);
            //     print!(" {} ",file.green());
            //     counter = counter + 1;
            // }
        // }
