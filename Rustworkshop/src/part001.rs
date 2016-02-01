fn find_item(v:Vec<i32>, num:i32) -> usize {
	for i in 0..v.len()-1 {
		if num == v[i] {
			return i;
		}
	}
	0
}

pub fn main() {
	let v = vec![5,6,7,8];
	let idx = find_item(v,7); // return positon
	println!("Position is {}", idx);
}
