use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let shared_data = std::sync::Arc::new(data);

    let handles: Vec<_> = (0..5).map(|i| {
        let shared_data = shared_data.clone();
        thread::spawn(move || {
            let local_sum: i32 = shared_data.iter().sum();
            println!("Thread {} Sum: {}", i, local_sum);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}


// Thread 0 Sum: 15
// Thread 2 Sum: 15
// Thread 4 Sum: 15
// Thread 1 Sum: 15
// Thread 3 Sum: 15
