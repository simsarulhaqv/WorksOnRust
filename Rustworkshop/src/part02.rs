enum SomethingOrNothing<T> {
	Something(T),
	Nothing
}

type PositionOrNothing = SomethingOrNothing<usize>;
// type IntegerOrNothing = SomethingOrNothing<i32>;


use self::SomethingOrNothing::{Something, Nothing};

fn find_item(v:Vec<i32>, num:i32) -> PositionOrNothing {
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
	match idx {
		Something(e) => { println!("Position is {}", e); },
 		Nothing => { println!("Not found"); }
	}
}
