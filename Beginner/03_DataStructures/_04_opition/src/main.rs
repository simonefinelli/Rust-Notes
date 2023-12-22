
/*
    In Rust we can use the 'option' enum to avoid to explicit set a value.

    The Option enum has the following definition in the std library:
    enum Option<T> {
        None,  // absence of value
        Some(T)
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

    let grade = get_grade(&"Frodo".to_string(), &student_db);
    match grade {
        Some(g) => println!("The grade is: {g}"),
        None => {}
    }

    // with a different syntax
    if let Some(g) = grade {
        println!("The grade is: {g}");
    }

}

fn get_grade(name: &String, db: &Vec<Student>) -> Option<u32> {
    for student in db {
        if student.name == *name {
            return student.grade;
        }
    }
    None
}
