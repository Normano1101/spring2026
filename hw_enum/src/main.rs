use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

fn perform_operation(operation: FileOperation) {
    match operation {

        FileOperation::List(path) => {
            Command::new("ls")
                .arg(path)
                .status()
                .expect("Failed to execute ls");
        }

        FileOperation::Display(file) => {
            Command::new("cat")
                .arg(file)
                .status()
                .expect("Failed to execute cat");
        }

        FileOperation::Create(file, content) => {
            let command = format!("echo '{}' > {}", content, file);

            Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .expect("Failed to create file");

            println!("File created successfully.");
        }

        FileOperation::Remove(file) => {
            Command::new("rm")
                .arg(file)
                .status()
                .expect("Failed to remove file");

            println!("File removed successfully.");
        }

        FileOperation::Pwd => {
            Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");
        }
    }
}

fn main() {

    println!("Welcome to the File Operations Program!");

    loop {

        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        print!("Enter your choice (0-5): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {

            "1" => {
                print!("Enter directory path: ");
                io::stdout().flush().unwrap();

                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();

                perform_operation(FileOperation::List(path.trim().to_string()));
            }

            "2" => {
                print!("Enter file path: ");
                io::stdout().flush().unwrap();

                let mut file = String::new();
                io::stdin().read_line(&mut file).unwrap();

                perform_operation(FileOperation::Display(file.trim().to_string()));
            }

            "3" => {
                print!("Enter file path: ");
                io::stdout().flush().unwrap();

                let mut file = String::new();
                io::stdin().read_line(&mut file).unwrap();

                print!("Enter content: ");
                io::stdout().flush().unwrap();

                let mut content = String::new();
                io::stdin().read_line(&mut content).unwrap();

                perform_operation(FileOperation::Create(
                    file.trim().to_string(),
                    content.trim().to_string(),
                ));
            }

            "4" => {
                print!("Enter file path: ");
                io::stdout().flush().unwrap();

                let mut file = String::new();
                io::stdin().read_line(&mut file).unwrap();

                perform_operation(FileOperation::Remove(file.trim().to_string()));
            }

            "5" => {
                perform_operation(FileOperation::Pwd);
            }

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid option. Try again.");
            }
        }
    }
}