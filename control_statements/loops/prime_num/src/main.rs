fn main() 
{
	let mut i = 2;
	let num = 7;
	loop
	{
		if num % i == 0
		{
			break;

		}
		i=i+1;


	}
	
	if i==num
	{
		println!("Num is prime {num}");
	}

}
