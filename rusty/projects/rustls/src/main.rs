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
    let mut counter = 1;
    let binding = get_long(&content).unwrap();
    let longest = binding.to_str().expect("REASON");
    let mut long: usize;
    if (content.len() > width as usize/longest.len()) {
        long = longest.len();
    } else {
        long = 5;
    }
    let mut curlen = 0;



    for path in content.iter() {
        // if counter % 8 == 0 {// || (content.len() < 6 && counter % 2 == 0 ){
        if curlen + long >= width.into() {
            print!("\n");
            curlen = long +2;
        }

        match Some(path.is_dir()) {
            Some(true) => {
                let Some(direc) = path.to_str() else { todo!()};
                // direcs.push(direc);
                // print!(" {:} ",direc.expect("REASON").yellow());
                
                print!("{:<a$}  ",direc[2..direc.len()].blue(),a=long+2);
                curlen = curlen + long;
                counter = counter + 1;

            },
            Some(false) => {
                let Some(file) = path.to_str() else { todo!()};
                // files.push(file);
                // print!("{:<width$} ",file,width=long);
                print!("{:<a$}  ",file[2..file.len()].red(),a=long+2);
                curlen = curlen + long;
                counter = counter + 1;

            },
            None => todo!()
        }
    }
        print!("\n");
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
