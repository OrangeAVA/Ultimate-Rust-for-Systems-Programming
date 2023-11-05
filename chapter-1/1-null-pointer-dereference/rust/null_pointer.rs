fn main() {
	let ptr: *const i32 = std::ptr::null();
	let value = unsafe { *ptr };
	println!("Value: {}", value);
}

// rustc null_pointer.rs && ./null_pointer 
// Segmentation fault (core dumped)