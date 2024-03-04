use std::{fs::{self, File}, path::PathBuf};
use reqwest;

struct Colors;

impl Colors {
    const RED: &'static str = "\x1b[31m";
    const BLUE: &'static str = "\x1b[34m";
    const RESET: &'static str = "\x1b[0m";
    const BOLD: &'static str = "\x1b[1m";
}

pub fn help() {
    println!("help: list all commands");
    println!("touch: create file");
    println!("rm: removes file");
    println!("mkdir: creates directory");
    println!("rmdir: removes directory");
    println!("ls: list files in current directory");
    println!("xor: XOR encryption");
    println!("clear: clears the screen");
    println!("pwd: prints current working directory");
    println!("curl: send get request");
    println!("exit: exit the program");
}

pub fn touch(file_name: &str) {
    match File::create(file_name) {
        Ok(_) => {
            println!("File created successfully: {}", file_name); 
        },
        Err(error) => {
            eprintln!("Problem creating the file: {:?}", error);
            return;
        }
    };
}

pub fn rm(file_name: &str) {
    match std::fs::remove_file(file_name) {
        Ok(_) => {
            println!("File removed successfully: {}", file_name); 
        },
        Err(error) => {
            eprintln!("Problem removing the file: {:?}", error);
            return;
        }
    };
}

pub fn mkdir(dir_name: &str) {
    match std::fs::create_dir(dir_name) {
        Ok(_) => {
            println!("Directory created successfully: {}", dir_name); 
        },
        Err(error) => {
            eprintln!("Problem creating the directory: {:?}", error);
            return;
        }
    };
}

pub fn rmdir(dir_name: &str) {
    match std::fs::remove_dir(dir_name) {
        Ok(_) => {
            println!("Directory removed successfully: {}", dir_name); 
        },
        Err(error) => {
            eprintln!("Problem removing the directory: {:?}", error);
            return;
        }
    };
}

pub fn ls() {
    let files: fs::ReadDir = match fs::read_dir(".") {
        Ok(files) => files,
        Err(error) => {
            eprintln!("Problem reading the directory: {:?}", error);
            return;
        }
    };

    for file in files {
        if let Ok(entry) = file {
            let file_path: std::path::PathBuf = entry.path();
            let display_file: std::path::Display<'_> = file_path.display();
            let str_display_file: String = display_file.to_string();
            let sliced_str_display_file: String = str_display_file[2..].to_string();

            // Check if the entry is a file or a directory
            if let Ok(metadata) = fs::metadata(&file_path) {
                if metadata.is_file() {
                    println!("{}{}{}{}", Colors::RED, Colors::BOLD, sliced_str_display_file, Colors::RESET);
                } else if metadata.is_dir() {
                    println!("{}{}{}{}", Colors::BLUE, Colors::BOLD, sliced_str_display_file, Colors::RESET);
                } else {
                    println!("{}", sliced_str_display_file);
                }
            } else {
                eprintln!("Error getting metadata for {}", sliced_str_display_file);
            }
        } else {
            eprintln!("Error reading directory entry");
        }
    }
}

pub fn xor(x_string: &str, key: char) -> String {
    let mut result: String = String::new();
    for c in x_string.chars() {
        let xor_result: char = (c as u8 ^ key as u8) as char;
        result.push(xor_result);
    }
    result
}

pub fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn pwd() -> Option<PathBuf> {
    match std::env::current_dir() {
        Ok(c_d) => Some(c_d),
        Err(e) => {
            eprintln!("Error: {}", e);
            None
        }
    }
}

pub async fn curl(url: &str) -> Result<String, reqwest::Error> {
    let response: reqwest::Response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}