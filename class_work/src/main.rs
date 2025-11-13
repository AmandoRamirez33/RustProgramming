trait ShowInfo {
    fn show_info(&self);
}

struct Undergrad{
   name:String,
   major:String,
   gpa:f32,
}

impl ShowInfo for Undergrad{
   fn show_info(&self) {
        println!(
            "Undergraduate Student: {}\nMajor: {}\nGPA: {:.2}\n",
            self.name, self.major, self.gpa
        );
    }
}

struct Grad{
   name: String,
   major: String,
   gpa: f32,
   thesis: String,
}

impl ShowInfo for Grad {
   fn show_info(&self) {
        println!(
            "Graduate Student: {}\nMajor: {}\nGPA: {:.2}\nThesis: {}\n",
            self.name, self.major, self.gpa, self.thesis
        );
    }
}

struct Enrollment <T: ShowInfo> {
   students: Vec<T>,
}

impl<T: ShowInfo> Enrollment<T> {
    fn new() -> Self {
        Enrollment { students: Vec::new() }
    }

    fn add_student(&mut self, student: T) {
        self.students.push(student);
    }

    fn show_all(&self) {
        for s in &self.students {
            s.show_info();
        }
    }
}



fn main() {
    
    let u1 = Undergrad {
        name: "Amando".into(),
        major: "Computer Science".into(),
        gpa:3.5,
    };

    let g1 = Grad {
        name: "Jared".into(),
        major: "Psychology".into(),
        gpa:3.4,
        thesis: "Social Anxiety Disorder".into(),
    };

    let mut under_enrollment = Enrollment::new();
    under_enrollment.add_student(u1);

    let mut grad_enrollment = Enrollment::new();
    grad_enrollment.add_student(g1);

    println!("Undergraduate Students: ");
    under_enrollment.show_all();

    println!("Graduate Students: ");
    grad_enrollment.show_all();

}