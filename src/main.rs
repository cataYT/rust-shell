use std::io::{self, Write};
mod cmd_funcs;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Welcome to my shell!!");

    loop {
        let mut input_string: String = String::new(); // Clear input_string at the beginning of each iteration
        print!("Enter your command: ");
        std::io::stdout().flush().expect("failed to flush buffer");
        match io::stdin().read_line(&mut input_string) {
            Ok(_) => {
                let low_input_str: String = input_string.to_lowercase().trim().to_string();
                match low_input_str.as_str() {
                    "help" => cmd_funcs::help(),
                    "exit" => break,
                    "touch" => {
                        let mut file_name: String = String::new();
                        print!("Enter your file name to create: ");
                        std::io::stdout().flush().expect("failed to flush buffer");
                        std::io::stdin().read_line(&mut file_name).unwrap();
                        file_name = file_name.trim().to_string();
                        cmd_funcs::touch(&file_name);
                    }
                    "rm" => {
                        let mut file_name: String = String::new();
                        print!("Enter your file name to remove: ");
                        std::io::stdout().flush().expect("failed to flush buffer");
                        std::io::stdin().read_line(&mut file_name).unwrap();
                        file_name = file_name.trim().to_string();
                        cmd_funcs::rm(&file_name);
                    }
                    "mkdir" => {
                        let mut directory_name: String = String::new();
                        print!("Enter your directory name: ");
                        std::io::stdout().flush().expect("failed to flush buffer");
                        std::io::stdin().read_line(&mut directory_name).unwrap();
                        directory_name = directory_name.trim().to_string();
                        cmd_funcs::mkdir(&directory_name);
                    }
                    "rmdir" => {
                        let mut directory_name: String = String::new();
                        print!("Enter your directory name: ");
                        std::io::stdout().flush().expect("failed to flush buffer");
                        std::io::stdin().read_line(&mut directory_name).unwrap();
                        directory_name = directory_name.trim().to_string();
                        cmd_funcs::rmdir(&directory_name);
                    }
                    "ls" => cmd_funcs::ls(),
                    "xor" => {
                        let mut x_string: String = String::new();
                        print!("Enter your string: ");
                        std::io::stdout().flush().expect("failed to flush buffer");
                        std::io::stdin().read_line(&mut x_string).unwrap();
                        x_string = x_string.trim().to_string();

                        let mut key: String = String::new();
                        print!("Enter your key: ");
                        std::io::stdout().flush().expect("failed to flush buffer");
                        std::io::stdin().read_line(&mut key).unwrap();
                        key = key.trim().chars().next().unwrap().to_string();
                        let result: String = cmd_funcs::xor(&x_string, key.chars().next().unwrap());
                        println!("{}", result);
                    }
                    "pwd" => cmd_funcs::pwd(),
                    "clear" => cmd_funcs::clear(),
                    "curl" => {
                        let mut url: String = String::new();
                        print!("Enter your url: ");
                        std::io::stdout().flush().expect("failed to flush buffer");
                        std::io::stdin().read_line(&mut url).unwrap();
                        url = url.trim().to_string();
                        match cmd_funcs::curl(&url).await {
                            Ok(body) => println!("Response body: {}", body),
                            Err(error) => println!("Error: {}", error),
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
