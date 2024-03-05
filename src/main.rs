use std::io::{self, Write};
mod cmd_funcs;

macro_rules! flush {
    () => {
        std::io::stdout().flush().expect("failed to flush buffer");
    };
}

fn replace_home_with_tilde(path: &str) -> String {
    if path.starts_with("/home/cata") {
        return format!("~{}", &path["/home/cata".len()..]);
    }
    path.to_string()
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Welcome to my shell!!");

    loop {
        let mut input_string: String = String::new(); // Clear input_string at the beginning of each iteration
        print!("{}$: ", replace_home_with_tilde(&cmd_funcs::pwd().unwrap().display().to_string()));
        flush!();
        match io::stdin().read_line(&mut input_string) {
            Ok(_) => {
                let low_input_str: String = input_string.to_lowercase().trim().to_string();
                match low_input_str.as_str() {
                    "help" => cmd_funcs::help(),
                    "exit" => break,
                    "touch" => {
                        let mut file_name: String = String::new();
                        print!("Enter your file name to create: ");
                        flush!();
                        std::io::stdin().read_line(&mut file_name).unwrap();
                        file_name = file_name.trim().to_string();
                        match cmd_funcs::touch(&file_name) {
                            Ok(name) => println!("File {} created successfully", name),
                            Err(error) => eprintln!("Error creating file: {}", error),
                        }
                    }
                    "rm" => {
                        let mut file_name: String = String::new();
                        print!("Enter your file name to remove: ");
                        flush!();
                        std::io::stdin().read_line(&mut file_name).unwrap();
                        file_name = file_name.trim().to_string();
                        match cmd_funcs::rm(file_name.as_str()) {
                            Ok(name) => println!("File {} removed successfully", name),
                            Err(error) => eprintln!("Error removing file: {}", error),
                        }
                    }
                    "mkdir" => {
                        let mut directory_name: String = String::new();
                        print!("Enter your directory name: ");
                        flush!();
                        std::io::stdin().read_line(&mut directory_name).unwrap();
                        directory_name = directory_name.trim().to_string();
                        match cmd_funcs::mkdir(&directory_name) {
                            Ok(name) => println!("Directory {} created successfully", name),
                            Err(error) => eprintln!("Error creating directory: {}", error),
                        }
                    }
                    "rmdir" => {
                        let mut directory_name: String = String::new();
                        print!("Enter your directory name: ");
                        flush!();
                        std::io::stdin().read_line(&mut directory_name).unwrap();
                        directory_name = directory_name.trim().to_string();
                        match cmd_funcs::rmdir(&directory_name) {
                            Ok(name) => println!("Directory {} removed successfully", name),
                            Err(error) => eprintln!("Error removing directory: {}", error),
                        }
                    }
                    "ls" => cmd_funcs::ls(),
                    "xor" => {
                        let mut x_string: String = String::new();
                        print!("Enter your string: ");
                        flush!();
                        std::io::stdin().read_line(&mut x_string).unwrap();
                        x_string = x_string.trim().to_string();

                        let mut key: String = String::new();
                        print!("Enter your key: ");
                        flush!();
                        std::io::stdin().read_line(&mut key).unwrap();
                        key = key.trim().chars().next().unwrap().to_string();
                        let result: String = cmd_funcs::xor(&x_string, key.chars().next().unwrap());
                        println!("Your xor encrypted string is: {}", result);
                    }
                    "pwd" => {
                        let _pwd = cmd_funcs::pwd().unwrap();
                        let pwd: std::path::Display<'_> = _pwd.display();
                        println!("{:?}", pwd);
                    }
                    "clear" => cmd_funcs::clear(),
                    "curl" => {
                        let mut url: String = String::new();
                        print!("Enter your url: ");
                        flush!();
                        std::io::stdin().read_line(&mut url).unwrap();
                        url = url.trim().to_string();
                        match cmd_funcs::curl(&url).await {
                            Ok(body) => println!("Response body: {}", body),
                            Err(error) => println!("Error: {}", error),
                        }
                    }
                    "calc" => {
                        let mut num1: String = String::new();
                        let mut num2: String = String::new();
                        let mut operation: String = String::new();
                        print!("Enter number 1: ");
                        flush!();
                        std::io::stdin().read_line(&mut num1).unwrap();
                        print!("Enter number 2: ");
                        flush!();
                        std::io::stdin().read_line(&mut num2).unwrap();
                        print!("Enter operation: ");
                        flush!();
                        std::io::stdin().read_line(&mut operation).unwrap();
                        match cmd_funcs::calc(&operation, &num1, &num2) {
                            Ok(result) => println!("Result: {}", result),
                            Err(error) => eprintln!("Error: {}", error),
                        }
                    }
                    _ => {
                        println!("Unknown command: {}", low_input_str);
                    }
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
            }
        }
    }
    Ok(())
}
