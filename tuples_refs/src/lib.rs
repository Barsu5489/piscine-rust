pub struct Student(pub u32, pub String, pub String);

// Function to return the id of the student
pub fn id(student: &Student) -> u32 {
    student.0
}

// Function to return the first name of the student
pub fn first_name(student: &Student) -> &str {
    &student.1
}

// Function to return the last name of the student
pub fn last_name(student: &Student) -> &str {
    &student.2
}


