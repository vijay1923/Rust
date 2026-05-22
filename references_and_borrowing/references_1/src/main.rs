
fn main() 
{
    let mut  s1 = String::from("hello");  // s should be mutable to change 

    // let len = calculate_length(&s1);  // pass a reference to the string
    // println!("After calculate length string is s1}' is : {len}.");

    change(&mut s1);  // pass a mutable reference to the string
    println!("After change: {s1}");
}

fn calculate_length(s: &String) -> usize 
{
    s.len() 
}  /* Here, s goes out of scope. But because s does not have ownership of what
   it refers to, the String is not dropped. */
 
 fn  change(s: &mut String)   // s is a mutable reference to a String
 {
     s.push_str(", world");  // push_str() appends a literal to a String
 }