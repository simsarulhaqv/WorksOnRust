
use part03::{SomethingOrNothing,Print};
use part03::SomethingOrNothing::*;

type NumberOrNothing = SomethingOrNothing<i32>;
//type FloatOrNothing = SomethingOrNothing<f32>;

trait Maximum {
	fn max(self, o:Self) -> Self;
}

impl Maximum for i32 {
	fn max(self, o:i32) -> i32 {
		if self>o {self} else {o}
	}
}

//impl Maximum for f32 {
//	fn max(self, o:f32) -> f32 {
//		if self>o {self} else {o}
//	}
//}


impl Print for NumberOrNothing {
	fn print(self) {
		match self {
			Something(e) => { println!("Number {}", e);},
			Nothing => { println!("Nothing"); }
		}
	}
}

//impl Print for FloatOrNothing {
//	fn print(self) {
//		match self {
//			Something(e) => { println!("Float {}", e);},
//			Nothing => { println!("Nothing"); }
//		}
//	}
//}

fn find_max(v:Vec<i32>) -> NumberOrNothing {
	let mut max = Nothing;
	for i in v {
		max  = Something(
			match max { Something(e) => i.max(e),
				    Nothing => i
			}
		       ); 
	}
	max
}

fn find_max_generic<T:Maximum>(v:Vec<T>) -> SomethingOrNothing<T> {
	let mut max = Nothing;
	for i in v {
		max = Something(match max {
				Something(e) => i.max(e),
				Nothing => i
		});
	}
	max
}

pub fn main() {
	let v = vec![4,10,6,7];
	let m = find_max_generic(v);
	m.print();
}
