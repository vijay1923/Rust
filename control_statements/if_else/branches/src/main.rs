fn main() 
{

    // if-else statement with multiple branches
    // let number = 10;  
    // if 5 > number        // if statement with condition 
    // {
    //     println!("Hello, world!");
    // }
    // else
    // {
    //     println!("bye , world!");
    // }


    // /************  even-odd number take input from user  *********************/
    // let mut input = String::new();
    // println!("Enter the number to check number is even or odd :");
    // std::io::stdin().read_line(&mut input).expect("Failed to read input");
    // let input: i32 = input.trim().parse().expect("Invalid input");

    // if input % 2 == 0
    // {
    //     println!("{} is an even number",input);
    // }
    // else    
    // {
    //     println!("{} is an odd number",input);
    // }



    // /********************  nested if else ******************/

    // let number = 15;
    // if number > 0 
    // {
    //     if number % 2 == 0 
    //     {
    //         println!("Positive even number");
    //     } 
    //     else 
    //     {
    //         println!("Positive odd number");
    //     }
    // } 
    // else if number < 0 
    // {
    //     println!("Negative number");
    // } 
    // else 
    // {
    //     println!("Number is zero");
    // }



    /******            *///////////////

    let condition = false;
    let number = if condition
    { 
        100    /// return on condition true 
    } 
    else 
    { 
        2000
    };
    println!("The value of number is: {}",number);







}
