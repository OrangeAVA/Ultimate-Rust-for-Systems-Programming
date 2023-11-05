fn main() {
    let mut data = [1, 2, 3, 4, 5];
    let data_ptr = data.as_mut_ptr();

    unsafe {
        *data_ptr.offset(2) = 10;
    }

    println!("Modified data: {:?}", data);
}

// Modified data: [1, 2, 10, 4, 5]
