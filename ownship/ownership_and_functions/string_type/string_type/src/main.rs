
fn main()
{

    // String type in Rust  to understand ownership and mutability
    println!("################### String litral example #####################");
    let mut s = String::from("Hello");  // string literal converts to string type using from function
    println!("before : {}",s);  
    s.push_str(" World");     // push_str is used to append string to the existing string
    println!("after : {}",s);
    println!("------------------------------------------------------------");

    // variaable and data interpretation with move 
    println!("########### variaable and data interpretation with move #############");
    let x=5;
    let y=x;  // here x is copied to y because i32 is a primitive type and implements the Copy trait
    println!("x: {}, y: {}", x, y);  // both x and y can be used after the assignment
    println!("------------------------------------------------------------");


    // Lets  understand by more complex data type 
    println!(" ########## Understand with the string type ##############");
    let  mut s1=String::from("hello");   // create mutable variable for string 1
    let s2=s1; // move to the s2 

    //println!("s1 : {}  s2 : {}",s1,s2);  // try to print  both the strings - error 
    //println!("S1:{}",s1);    /// errror - now s2 have the ownership of string 

    println!("S2:{}",s2);   // s2 has ownership of string 
    s1=s2;   //  ownership changed to s1 again 
    println!("S1 : {}",s1);  // ownership back to s1 

    




}
