struct Student {
    name: String,
    age: i32,
    address: String,
}

fn get_student () -> Option<Student> {
    let student = Student {
        name: "Gerry".to_string(),
        age: 15,
        address: "Jakarta".to_string()
    };

    Some(student)
}

fn main() {
    let student = get_student();

    match student {
        Some(s) => println!("Name: {:?}", s.name),
        None => println!("No student found!"),
    }
}

