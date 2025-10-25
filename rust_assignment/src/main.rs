use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    let mut file = match File::create(filename){
        Ok(f) => f,
        Err(e) => {
            eprint!("Failed to create file:{}", e);
            return;
        }
    };

    for book in books {
        if let Err(e) = writeln!(file, "{},{},{}", book.title, book.author,book.year){
            eprintln!("Failed to write to file: {}", e);
            return;
        }
    }
}

fn load_books(filename: &str) -> Vec<Book> {
   let file = match File::open(filename){
    Ok(f) => f,
    Err(e) => {
        eprintln!("Failed to open file: {}", e);
        return Vec::new();
    }
   };

   let reader = BufReader::new(file);
   let mut books = Vec::new();

   for line in reader.lines(){
    match line {
        Ok(line_content) => {
            let parts: Vec<&str> = line_content.split(',').collect();
            if parts.len() == 3{
                let title = parts[0].to_string();
                let author = parts[1].to_string();
                let year = parts[2].parse::<u16>().unwrap_or(0);
                books.push(Book {title, author, year});
            }
        }
        Err(e) => {
            eprintln!("Error reading line: {}", e);
        }
    }
   }
   books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}