
fn main()
{
  let x=1;
  let y=x;
  println!("x={x},y={y}");
  let z=y.clone();
  println!("x={x},y={y},z={z}");

   println!("address of x = {:p}", &x); 
    println!("address of y = {:p}", &y);
     println!("address of z = {:p}", &z);
  

   
}