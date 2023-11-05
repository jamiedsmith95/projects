use colored::*;
use crossterm::cursor;
use regex::Regex;
use std::{
    process::exit,
    env::{args, Args},
    fmt::{self, format},
    fs, io,
    ops::Deref,
    path::{Path, PathBuf, self},
    usize, vec,
};
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

fn get_termsize() -> (u16, u16) {
    termion::terminal_size().unwrap()
}

fn get_contents(binding: String) -> Vec<PathBuf> {
    //get the contents of the desired directory

    let pathb = Path::new(&binding);
    // println!("{:?}",&pathb.file_name());
    let pattern = format!(r"{:?}", &pathb.file_name());
    let mut expression = Regex::new(&pattern).unwrap();
    // match pathb.file_name() {
    //     Some(name) => {
    //         expression = Regex::new(format!("{:?}{:?}",name,".*").as_str()).unwrap();
    //     },
    //     None => expression = Regex::new("\\s?.*").unwrap()
    // }

    // println!("{:?}", expression);
    if fs::read_dir(pathb).is_err(){
        // println!("No results found");
        if pathb.to_path_buf().exists() {

        let return_file = vec![pathb.to_path_buf()];
        return return_file
        } else {
            return Vec::new()
        }
    } else {
    return fs::read_dir(pathb)
        .unwrap()
        .filter_map(|e| e.ok())
        // .filter(|e| expression.is_match(&e.file_name().to_str().unwrap()))
        .map(|e| e.path())
        .collect::<Vec<PathBuf>>()
    }
}

// fn get_args
fn get_loc() -> Vec<String> {
    let my_args = args().collect::<Vec<String>>();
    match my_args.to_vec().len() >= 2 {
        true => {
            return my_args[1..my_args.len()].to_vec();
            // return lastarg
        } // return current dir
        false => return vec!["./".to_owned()],
    }
}

fn get_long(content: &Vec<PathBuf>) -> Option<PathBuf> {
    let pathb = PathBuf::from(" ");

    if content.len() == 0 {
        return None;
    } else {
        let long = content.iter().fold(pathb, |acc, item| {
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

fn itemprint(item: &Item, long: usize) {
    let mut theitem: String = item.name.to_owned();
    // theitem.truncate(long).to_owned();
    if item.name.len() > long {
        theitem = theitem[0..long].to_string();
    }
    match item.ford {
        Ford::File => print!("{:long$}  ", theitem.cyan()), //,width = long),
        Ford::Dir => print!("{:<long$}  ", theitem.purple()), //,width = long),
    }
}

fn sort(content: &Vec<PathBuf>) -> Vec<Item> {
    let mut all: Vec<Item> = Vec::new();
    for path in content.iter() {
        match Some(path.is_dir()) {
            Some(true) => {
                let Some(direc) = path.file_name().expect("").to_str() else {
                    todo!()
                };
                all.push(Item {
                    ford: Ford::Dir,
                    name: direc.to_string(),
                });
            }
            Some(false) => {
                let Some(file) = path.file_name().expect("").to_str() else {
                    todo!()
                };
                all.push(Item {
                    ford: Ford::File,
                    name: file.to_string(),
                });
            }
            None => todo!(),
        }
    }
    return all;
}
fn checkspace(item: &Item, long: usize) {
    let (width, _) = get_termsize();
    let Ok((x, _y)): Result<(u16, u16), io::Error> =
        std::io::stdout().into_raw_mode().unwrap().cursor_pos()
    else {
        todo!()
    };
    if (width - x) <= long as u16 {
        print!("\n");
        itemprint(item, long)
    } else {
        itemprint(item, long)
    }
}

fn main() {
    let locations = get_loc();
    let mut content = vec![];
    for loc in locations {
        content.append(&mut get_contents(loc))
    }
    // println!("{}",&content.len());
    if let Some(binding) = get_long(&content) {
        let longest = binding.to_str().unwrap();
        let long = match longest.len() > 20 {
            true => 20,
            false => longest.len(),
        };
        let all = sort(&content);
        for item in all.iter() {
            checkspace(item, long);
            // itemprint(item,long);
        }
        print!("\n")
    } else {
        let longest = "a".repeat(20);
        let long = match longest.len() > 20 {
            true => 20,
            false => longest.len(),
        };
        let all = sort(&content);
        for item in all.iter() {
            checkspace(item, long);
            // itemprint(item,long);
        }
        print!("\n")

    }
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
