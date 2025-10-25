use std::process::Command;
use std::io::{self,Write};
use std::fs::File;

fn reading_from_console(prompt: &str) -> String {
    let mut buffer = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
   
}
fn read_file_linux(filename: &str){
    let output = Command::new("cat")
        .arg(&filename)
        .output()
        .expect("Failed execution");
    println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
}
fn create_and_write_to_file(filename: &String) {
    let mut file = File::create("example.txt").unwrap();
    writeln!(file, "Hello, Rust file operations!").unwrap();
    writeln!(file, "This is a new line.").unwrap();
    println!("File '{}' has been successfully created and written.", filename);
}
fn main(){
    loop {
        println!("Type 1, 2, or 3 to Create a file, Read a file, or Close, respectively");

        let choice: i32 = reading_from_console("Type the number: ")
        .parse()
        .unwrap_or(0);

        match choice {
            1 => {
                let filename = reading_from_console("What file to create? ");
                create_and_write_to_file(&filename);
            }
            2 => {
                let filename = reading_from_console("What file to open? ");
                read_file_linux(&filename);
            }
            3 => {
                println!("Goodbye.");
                break;
            }

            _ => println!("Invalid choice."),
        }
    }
       
}