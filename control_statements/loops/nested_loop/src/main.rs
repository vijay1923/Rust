fn main() 
{
 let mut count_up = 0;
    let mut count_down = 10;

    'uoter:loop
    {
         count_up =count_up + 1;
        println!("Count up: {}", count_up);
    
        'inner: loop
        {
            
            println!("Count down: {}", count_down);
            count_down = count_down - 1;

            if count_up == 10 
            {
                break 'uoter; // Breaks out of the outer loop
            }
            if count_down > 0
            {
                break ; // Breaks out of the inner loop
            }
        }
        

    }

    
}
