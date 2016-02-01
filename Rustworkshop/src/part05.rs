use std::io::prelude::*;
use std::io;
use part03::{find_item, Print};

fn read_vec() -> Vec<i32> {
	let mut ret = Vec::<i32>::new();
	let stdin = io::stdin();
	println!("Enter a lsit of numbers one by one. ^D to end");
	
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		match line.trim().parse::<i32>() {
			Ok(n) => {ret.push(n);},
			Err(_) => { println!("Need for numbers");}
		}
	}
	ret	
}

pub fn main() {
	let v = read_vec();
	let idx = find_item(v, 3);
	idx.print();
}
