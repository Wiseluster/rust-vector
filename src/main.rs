use vector::*;

fn main()
{
	let a = Vector::at(3, 4, 2);
	let b = Vector::at(5, -1, 0.5);

	println!("a = {}", a);
	println!("b = {}", b);
	println!("a + b = {}", a + b);
	println!("a - b = {}", a - b);
	println!("a * b = {}", a * b);
	println!("a * 2 = {}", a * 2);
	println!("a % b = {}", a % b);
	println!("-a = {}", -a);
	println!("!a = {}", !a);
}
