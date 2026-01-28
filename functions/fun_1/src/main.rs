/// this program demonstrates functions in Rust
// arguments and parameters and their usage and behavior 
// passing values to functions and returning values from functions


fn main() 
{
    println!("Hello, world! From main ");
   let x = another_function(5);  // pass 5 as argument to another_function and receive returned value in x 
    
    println!("Back to main function."); 
    // catch the returned value from another_function
    println!("The returned value from another_function is: {x}");   // print the returned value 

}
fn another_function(x: i32) -> i32
{
    let y: i32 = 10;
    println!("From another function.");
    println!("The value of x is: {} and value of y is  {y}", x);
    println!("{1} {0}", x, y,);
    println!("val= {val}", val = x);
     y // return y 


}   