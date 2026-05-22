fn main() 
{
    
    let s=String::from("Ashitosh");
    //println!("value of s : {s}");
    let s1=s.clone();
    println!("value of s1 is {s1} and value of s1 is {s1}");
    
    println!("address of s = {:p}", &s);
    println!("address of s1 = {:p}", &s1);

  
}
