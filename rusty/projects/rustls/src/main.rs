use std::{env::{args, Args},fmt::{self, format}, fs, io, path::{Path,PathBuf}, vec, ops::Deref, usize};
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
    let binding = get_loc();
    let pathb = Path::new(&binding);
    fs::read_dir(pathb)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect::<Vec<_>>() // put this into a match for whether user included
}

// fn get_args
//
fn get_loc() -> String {
    let my_args = args().collect::<Vec<String>>();
    println!("{:?}",my_args.len());
    match &my_args.len() {
        2 => {
           return my_args[1].as_str().to_owned();
           // return lastarg

        }
        _ => return "./".to_string(),

}}

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

fn itemprint(item: &Item, long: usize) {
    let (mut theitem): String = item.name.to_owned();
    // theitem.truncate(long).to_owned();
    if item.name.len() > long { 
        theitem = theitem[0..long].to_string();
    }
    match item.ford {
        Ford::File => print!("{:long$}  ", theitem.cyan()),//,width = long),
        Ford::Dir => print!("{:<long$}  ", theitem.purple()),//,width = long),
    }
}

fn sort(content: &Vec<PathBuf>) -> Vec<Item> {
    let mut all: Vec<Item> = Vec::new();
    for path in content.iter() {

        match Some(path.is_dir()) {
            Some(true) => {
                let Some(direc) = path.file_name().expect("").to_str() else { todo!()};
                all.push(Item { ford: Ford::Dir, name: direc.to_string()});

            },
            Some(false) => {
                let Some(file) = path.file_name().expect("").to_str() else { todo!()};
                all.push(Item { ford: Ford::File, name: file.to_string()});

            },
            None => todo!()
        }
    }
    return all
}
fn checkspace(item: &Item, long:usize) {
    let (width,_) = get_termsize();
    let Ok((x, y)): Result<(u16, u16), io::Error> = std::io::stdout().into_raw_mode().unwrap().cursor_pos() else { todo!() };
    if (width - x ) <= long as u16 {
        print!("\n");
        itemprint(item, long)
    } else {
        itemprint(item, long)
    }
}


fn main() {
    let content = get_contents();
    let binding = get_long(&content).unwrap();
    let longest = binding.to_str().expect("REASON");
    let long = match longest.len() > 15 {
        true => 15,
        false => longest.len(),
    };
    let all = sort(&content);
    for item in all.iter() {
        checkspace(item,long);
        // itemprint(item,long);
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
