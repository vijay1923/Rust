fn makes_copy(num: i32 ) // num comes into a scope 
{
    println!("From Ownership fun");
    println!("Num:{num}");
}  // num goes out of scope , nothing special happen 

fn takes_ownership(s:String) // s comes into scope 
{
    println!("From Ownership fun");
    println!("String : {s}");
} // s goes out of scope , drop is called and memory is freed

fn main() 
{
    let s = String::from("Ahitosh"); // s comes into scope 
    takes_ownership(s); // s value is moves intp function and s goes outod scope 
    /* Using s from this onwords is prohibited , rust will throw an error */
   //  println!("String in main: {s}"); // this will give you error 

   /*If want ot pass the s value to fuction and 
   still want to use it in mian then pass s.clone() into function*/


    let x = 5; // x comes into scope
    makes_copy(x);   // x still has its scope in main onwords due to copy trait 

    /* Using x from this onword is allowed */
    println!("X in main after copy trait: {x}");
}
