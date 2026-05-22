fn main() 
{
    // Step 1: Create a String on the heap
    // "hello" is stored in the heap
    // 's' (on the stack) holds pointer, length, and capacity
    let mut s = String::from("hello");

    println!("First value of s: {}", s);

    // Step 2: Reassign a new value to the same variable
    // Before assigning "ahoy", Rust automatically calls drop on the old value ("hello")
    // This frees the heap memory used by "hello"
    s = String::from("ahoy");

    // Now 's' owns a new String stored in the heap
    println!("After reassignment: {}", s);

    
    println!("Modified string: {}", s);

    // When main() ends, 's' goes out of scope
    // Rust automatically calls drop again
    // Heap memory for "ahoy sailor" is freed
}