
pub fn main(limit:usize) -> usize {
	let mut total = 0;
	let mut first = 0;
	let mut second = 1;
	let mut tempy = 0;
	while second <= limit {
		tempy = second;
		second += first;
		if second%2==0 {
			total = total + second;
			println!("Sum is {}",total);
			println!("number is  {}",second);
		}
		first = tempy;
	}
	println!("Sum is {}",total);
	total
}



