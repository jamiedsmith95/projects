use std::{env,fmt::{self, format}, fs, io, path::{Path,PathBuf}, vec, ops::Deref};
use colored::*;
use termion;

#[derive(Debug)]
enum Ford {
    File,
    Dir,
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

fn main() {
    let content = get_contents();
    // let mut files = Vec::new();
    // let mut direcs = Vec::new();
    let (width, height) = get_termsize();
    let mut counter = 0;
    let binding = get_long(&content).unwrap();
    let long = binding.to_str().expect("REASON");
    println!("{}",&long);



    for path in content.iter() {
        if counter % 5 == 0 || (content.len() < 6 && counter % 2 == 0 ){
            // print!("\n");
        }

        match Some(path.is_dir()) {
            Some(true) => {
                let Some(direc) = path.to_str() else { todo!()};
                // direcs.push(direc);
                // print!(" {:} ",direc.expect("REASON").yellow());
                let formatted = format!("{: <{a}}",a=long, &direc);
                print!(" {} ",formatted);
                counter = counter + 1;

            },
            Some(false) => {
                let Some(file) = path.to_str() else { todo!()};
                let formatted = format!("{: <{a}}",a=long, &file);
                // files.push(file);
                print!(" {} ",formatted);
                counter = counter + 1;

            },
            None => todo!()
        }
    }
    println!("\n")
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
