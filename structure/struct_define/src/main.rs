// This program defines a structure to store student information and demonstrates how to create an instance of the structure and access its fields.


struct strudent // structure to store student information
{
    roll_no: i32,
    name: String,
    marks: f32, 
    grade: char,
    year_of_passing: i32,
}

fn main() 
{

   println!("Enter the student record:");
   println!("Enter the roll number:");
   let mut roll_no = String::new();
   std::io::stdin().read_line(&mut roll_no).expect("Failed to read line");
   let roll_no: i32 = roll_no.trim().parse().expect("Please enter a valid number");
   println!("Enter the name:");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();
    println!("Enter the marks:");
    let mut marks = String::new();
    std::io::stdin().read_line(&mut marks).expect("Failed to read line");
    let marks: f32 = marks.trim().parse().expect("Please enter a valid number");
    println!("Enter the grade:");
    let mut grade = String::new();
    std::io::stdin().read_line(&mut grade).expect("Failed to read line");
    let grade: char = grade.trim().chars().next().expect("Please enter a valid character");
    println!("Enter the year of passing:");
    let mut year_of_passing = String::new();
    std::io::stdin().read_line(&mut year_of_passing).expect("Failed to read line");
    let year_of_passing: i32 = year_of_passing.trim().parse().expect("Please enter a valid number");
    println!("\nStudent Record:");
    println!("Roll Number: {}", roll_no);
    println!("Name: {}", name);
    println!("Marks: {}", marks);
    println!("Grade: {}", grade);
    println!("Year of Passing: {}", year_of_passing);   
    println!("\nCreating a student record using the structure...");

    let student1 = strudent  // creating an instance of the structure and initializing it with user input
    {
        roll_no,
        name,
        marks,
        grade,
        year_of_passing,
    };

    println!("\nStudent Record:");
    println!("Roll Number: {}", student1.roll_no);
    println!("Name: {}", student1.name);
    println!("Marks: {}", student1.marks);
    println!("Grade: {}", student1.grade);
    println!("Year of Passing: {}", student1.year_of_passing);

   
}
