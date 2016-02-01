
pub fn main(a:i32, b:i32, limit:i32) -> i32 {
	let mut total = 0;
	for i in 1..limit {
		if i%3==0 { 
			total += i;
		 } 
		else if i%5==0 {
			total += i;
		}
	}
	println!("Sum i s {}",total);
	total
}



