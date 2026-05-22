// scan number from user and print its binary 
fn main() 
{
    let mut num = String::new(); // create a mutable string to store user input
    let mut pos = 0; // initialize position to 0
    println!("Enter a number : ");
    std::io::stdin().read_line(&mut num).expect("Failed to read line");
    let num: u32 = num.trim().parse().expect("Please type a number!");
    println!("Binary representation of {} is : ",num);
    while 32>pos
    {
       print!("{}",num>>pos&1); // right shift num by pos and bitwise AND with 1 to get the bit at pos
       if pos%4==3 // print nibble by niblle
       {
        print!(" ");
       }
       pos+=1; // increment position
       
    }
    println!();
}

