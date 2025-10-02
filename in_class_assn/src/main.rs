struct Student {
    name: String,
    major: String,
}

impl Student {
    fn new(name: &str, major:&str) -> Student {
        Student {
            name : name.to_string(),
            major: major.to_string(),
        }
    }
    fn set_major(&mut self, new_major:&str){
        self.major = new_major.to_string();
    }
    fn get_major(&self) -> &str{
        &self.major
    }
}

fn main() {
    let mut student1 = Student::new("Amando", "Computer Science");
    println!("Student Name: {}", student1.name);
    println!("Current Major: {}", student1.get_major());
    student1.set_major("Cyber Security");
    println!("New Major: {}", student1.get_major());
}