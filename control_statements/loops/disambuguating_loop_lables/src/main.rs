// this program demonstrates how to disambiguate loop labels in Rust.
// In Rust, you can label loops to specify which loop you want to break out of or continue. This is especially useful when you have nested loops. In this example, we have an outer loop labeled 'counting_up and an inner loop without a label. We use the label to break out of the outer loop when a certain condition is met.
// The program will print the count and remaining values, and it will break out of the inner loop when remaining equals 9, and it will break out of the outer loop when count equals 2.

fn main() 
{
    let mut count = 0; // Initialize count to 0
    'counting_up: loop  // outer loop will break when count is 2
    {
        println!("count = {count}");  
        let mut remaining = 10;   // Initialize remaining to 10 for each iteration of the outer loop

        loop  // iiner loop will run until the conut is 2 or remaining is 9
        {
            println!("remaining = {remaining}");
            if remaining == 9  // break the inner loop when remaining is 9
            {
                break;
            }
            if count == 2 
            {
                break 'counting_up;  // break the ourter loop when count is 2
            }
            remaining -= 1;  // Decrement remaining by 1 for the next iteration of the inner loop
        }

        count += 1; // Increment count by 1 for the next iteration of the outer loop
    }
    println!("End count = {count}");
}