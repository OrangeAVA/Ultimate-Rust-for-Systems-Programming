fn main() {
	let mut vec = vec![1, 2, 3];
	// let mut vec: Vec<i32> = vec![];
	let item = vec.pop();
	match item {
		Some(val) => println!("Popped value: {}", val),
		None => println!("Vector is empty"),
	}
}

// rustc null_pointer.rs 
// Output: Popped value: 3