use std::thread;

fn main() {
	let data = vec![1, 2, 3, 4, 5];
	let mut handles = vec![];

	for &item in &data {
		handles.push(thread::spawn(move || {
			println!("Processed: {}", item * 2);
		}));
	}

	for handle in handles {
		handle.join().unwrap();
	}

}

//  rustc thread.rs && ./thread

// Processed: 2
// Processed: 6
// Processed: 4
// Processed: 8
// Processed: 10
