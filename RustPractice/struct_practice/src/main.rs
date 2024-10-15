
mod student;
use student::Student;

fn main() {
    let name = "John".to_string();
    let major = "Computer Science".to_string();

    let mut s = Student::new_student(name, major);

    println!("{:?}", s);

    s.introduce_yourself();
    s.change_major("Artificial Intelligence");
    s.introduce_yourself();
}
