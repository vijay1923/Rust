fn main() 
{
 
    let s1= String::from("Ashitosh");
    let (s2,len) = calculate_length(s1);

    println!("stirng : {s2} and string length is {len}");

    let len1=function_one(String::from("Nikam"));
    println!("lenght : {len1}");
}

fn calculate_length(s:String) -> (String,usize)
{
    let len=s.len();  // lengt of string  
    (s,len)     // return len to calling function 


}

fn function_one(s:String) -> usize
{

    s.len() // return string length 

}