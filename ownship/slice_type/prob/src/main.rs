/* Write a function that takes a string of words separated 
by spaces and returns the first word it finds in that string. 
If the function doesn’t find a space in the string, 
the whole string must be one word, so the entire string should be returned.
*/


fn main() 
{
    let word = first_word("hello world");
    println!("The first word is: {}", word);
}
