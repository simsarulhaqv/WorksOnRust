enum PositionOrNothing {
	Position(usize),
	Nothing
}


fn find_item(v:Vec<i32>, num:i32) -> PositionOrNothing {
	for i in 0..v.len()-1 {
		if num == v[i] {
			return PositionOrNothing::Position(i);
		}
	}
	PositionOrNothing::Nothing
}

pub fn main() {
	let v = vec![5,6,7,8];
	let idx = find_item(v,10); // return positon
	match idx {
		PositionOrNothing::Position(e) => { println!("Position is {}", e); },
 		PositionOrNothing::Nothing => { println!("Not found"); }
	}
}
