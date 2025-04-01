// Define the tuple struct to represent a Student
pub struct Student(u32, String, String);

// Function to get the student's id
pub fn id(student: &Student) -> u32 {
    student.0
}

// Function to get the student's first name
pub fn first_name(student: &Student) -> &str {
    &student.1
}

// Function to get the student's last name
pub fn last_name(student: &Student) -> &str {
    &student.2
}
