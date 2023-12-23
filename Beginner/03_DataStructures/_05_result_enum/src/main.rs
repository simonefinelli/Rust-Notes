
/*
    In Rust we can use the 'result' enum to handle the success or fail of an execution of a function.

    In the STD library the result enum is defined as follow:
    enum Result<T, E> {
        Ok(T),  // holds a generic value
        Err(E)  // holds another generic value, representing the details of the error
    }

 */

struct Student {
    name: String,
    grade: Option<u32>
}

fn main() {
    let student_db = vec![
        Student {
            name: String::from("Frodo"),
            grade: Some(55)
        },
        Student {
            name: String::from("Bilbo"),
            grade: None  // now we can set None value instead of putting some garbage
        }
    ];


    // check student in DB
    let status = get_student_grade(&"Gandalf".to_string(), &student_db);
    match status {
        Ok(grade) => {
            if let Some(g) = grade {
                println!("The grade is: {g}");
            } else {
                println!("Grade not available");
            }
        },
        Err(err_msg) => println!("An error occur: {err_msg}")
    }
}

fn get_student_grade(name: &String, db: &Vec<Student>) -> Result<Option<u32>, String> {
    for student in db {
        if student.name == *name {
            return Ok(student.grade);
        }
    }
    Err(String::from("Student not found!"))
}

