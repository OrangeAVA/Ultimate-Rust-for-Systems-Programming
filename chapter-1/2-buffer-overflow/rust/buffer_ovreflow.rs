fn main() {
	let mut buffer: [u8; 5] = [0; 5];
	let data = b"Overflowing Content!";
	buffer[..data.len()].copy_from_slice(data);
	println!("Buffer: {:?}", buffer);
}
// rustc buffer_ovreflow.rs && ./buffer_ovreflow
// thread 'main' panicked at 'range end index 20 out of range for slice of length 5', buffer_ovreflow.rs:4:5
