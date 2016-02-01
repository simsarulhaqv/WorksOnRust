// Strings are unicode unlike ascii in C/C++

fn print_me_2(s: String) {
	println!("I am {}",s);
}


pub  fn print_me(s:&str) {
	println!("I am {}", s);
}

pub fn main() {
	println!("hello world");
	let f = "simsar";
	print_me(&f);
	let f = "SIMSAR";
	print_me_2(f.to_string());
}
