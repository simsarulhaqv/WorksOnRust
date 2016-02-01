pub enum SomethingOrNothing<T> {
	Something(T),
	Nothing
}


use self::SomethingOrNothing::{Something, Nothing};


pub type PositionOrNothing = SomethingOrNothing<usize>; 

pub trait Print {
	fn print(self);
}

impl Print for PositionOrNothing {
	fn print(self) {
		match self {
			Something(e) => { println!("Position is {}", e); },
 			Nothing => { println!("Not found"); }
		}
	}
}

pub fn find_item(v:&Vec<i32>, num:i32) -> PositionOrNothing {
	for i in 0..v.len() {
		if num == v[i] {
			return Something(i);
		}
	}
	Nothing
}

fn sharing_demo() {
        let v = vec![5,6,7,8];
        let idx = find_item(&v,7); // return positon
        idx.print();
        let idx2 = find_item(&v,6);
        idx2.print();
}


fn increment(v:&mut Vec<i32>) {
	for i in v {
		*i += 1;
	}
}

fn mutation_demo() {
	let mut v = vec![4,5,6,7];
	{
		//let a = &v[0];
		let x = find_item(&v, 7);
		x.print();
	}
	increment(&mut v);
	for i in v {
		println!("Test {}", i);
	}
}

pub fn main() {
	sharing_demo();
	mutation_demo();
}


