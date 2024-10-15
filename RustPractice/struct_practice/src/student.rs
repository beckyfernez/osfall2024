
// define a struct strudent
//with name and major

//provide 3 methods
//create a new student
//change name
//change major

#[derive(Debug)]
pub struct Student {
    name: String,
    major: String,
}

impl Student {
    pub fn new_student(name:String,major:String) -> Self {
        Student {
        name,
        major,
        }
    }

    pub fn introduce_yourself(&self) {
        println!("Hey my name is {}. I am majoring in {}",self.name,self.major);
    }

    pub fn change_major(&mut self, new_major:&str){
        self.major = new_major.to_string();

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_creation() {
        let s = Student::new_student("Alex".to_string(),
                            "Computer Science".to_string());
        assert_eq!(s.name, "Alex".to_string());
    }

    //#[test]
    //fn test_major_creation() {
        //let s = Student::new_major("Alex".to_string(),
                            //"Computer Science".to_string());
        //assert_eq!(s.major, "Computer Science".to_string());
    //}
}