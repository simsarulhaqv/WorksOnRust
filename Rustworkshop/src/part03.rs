pub enum SomethingOrNothing<T> {
	Something(T),
	Nothing
}

pub type PositionOrNothing = SomethingOrNothing<usize>;
//type NumberOrNothing = SomethingOrNothing<i32>;

pub trait Print {
	fn print(self);
}

trait Is {
	fn is(self, o:Self) -> bool;
}

impl Is for i32 {
	fn is(self, o:Self) -> bool {
		self == o
	}
}

use self::SomethingOrNothing::{Something, Nothing};

impl Print for PositionOrNothing {
	fn print(self) {
		match self {
			Something(e) => { println!("Position is {}", e); },
 			Nothing => { println!("Not found"); }
		}
	}
}


//impl Print for NumberOrNothing {
//	fn print(self) {
//		match self {
//			Something(e) => { println!("Number {}", e);},
//			Nothing => { println!("Nothing"); }
//		}
//	}
//}

pub fn find_item(v:Vec<i32>, num:i32) -> PositionOrNothing {
	for i in 0..v.len()-1 {
		if num == v[i] {
			return Something(i);
		}
	}
	Nothing
}

pub fn main() {
	let v = vec![5,6,7,8];
	let idx = find_item(v,7); // return positon
	idx.print();

}
