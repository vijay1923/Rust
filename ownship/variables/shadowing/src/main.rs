// this example demonstrates variable shadowing in Rust.
// Variable shadowing allows you to declare a new variable with the same name as a previous variable.
// The new variable "shadows" the previous one, meaning that the previous variable is no
// longer accessible in the current scope.
// rust variables are immutable by default, but shadowing allows you to effectively create a new variable
// with the same name, which can be useful for transforming data without needing to come up with
// new variable names.
fn main() 
{
    let x :u32 = 5;

    let x = 100;

    {
        let x = x + 5;
        println!("The value of x in the inner scope is: {x}");
    }

    {
        let x = 10;
        let x = 12;
        let x = 20;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}