fn main() 
{
    let mut c=0;
    let result= loop
    {
        c=c+1;
        println!("Counter: {}", c);
        if c==10
        {
            break c*2;
        }

    };
    println!("last value of counter: {}", c);
    println!("result: {}", result);
}
