struct Resource {
	data: Vec<u8>,
}
fn main() {
	let resource = Resource {
	data: vec![1, 2, 3, 4, 5],
	};
}
// rustc buffer_ovreflow.rs && ./buffer_ovreflow
// thread 'main' panicked at 'range end index 20 out of range for slice of length 5', buffer_ovreflow.rs:4:5
