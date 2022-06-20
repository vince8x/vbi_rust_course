use std::collections::HashMap;

fn main() {
    print!("Exercise day 3");
}

pub struct School {
    students: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        School {
            students: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students.insert(String::from(student), grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = Vec::new();
        for (_key, value) in &self.students {
            if !grades.contains(value) {
                grades.push(*value);
            }
        }
        grades.sort();
        return grades;
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut student_names: Vec<String> = Vec::new();
        for (key, value) in &self.students {
            if grade == *value {
                student_names.push(String::from(key));
            }
        }
        student_names.sort();
        return student_names;
    }
}

#[test]
fn test_empty_school() {
    let school = School::new();
    assert_eq!(0, school.grades().len());
}

#[test]
fn test_grades_list() {
    let mut school = School::new();
    school.add(2, "Lee");
    let grades1 = school.grades();
    assert_eq!(1, grades1.len());
    assert_eq!(2, grades1[0]);

    school.add(3, "Nancy");
    let grades2 = school.grades();
    assert_eq!(2, grades2.len());
    assert_eq!(2, grades2[0]);
    assert_eq!(3, grades2[1]);
}

#[test]
fn test_find_grade() {
    let mut school = School::new();
    school.add(4, "Bob");
    school.add(4, "Alice");
    school.add(5, "Tom");

    let students_grade_4 = school.grade(4);
    println!("{:?}", students_grade_4);
    assert_eq!(2, students_grade_4.len());
    assert!(students_grade_4[0].eq("Alice"));
    assert!(students_grade_4[1].eq("Bob"));
}
