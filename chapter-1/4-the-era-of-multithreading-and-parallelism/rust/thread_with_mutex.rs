use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
	let data = Arc::new(Mutex::new(0));

	let handles: Vec<_> = (0..10)
		.map(|_| {
			let data = data.clone();
			thread::spawn(move || {
				let mut data = data.lock().unwrap();
				*data += 1;
			})
	})
	.collect();

	for handle in handles {
		handle.join().unwrap();
	}

	println!("Final data value: {:?}", *data.lock().unwrap());
}

// Final data value: 10