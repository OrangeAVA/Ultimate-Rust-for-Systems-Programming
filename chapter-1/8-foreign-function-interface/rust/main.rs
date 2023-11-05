extern "C" {
    fn process_data(data: *mut u8, length: usize);
}

fn main() {
    let mut data: Vec<u8> = vec![1, 2, 3, 4, 5];

    unsafe {
        process_data(data.as_mut_ptr(), data.len());
    }
}

// gcc -c external_lib.c -o external_lib.o
// ar rcs libexternal_lib.a external_lib.o

// rustc -L . -o main main.rs -l external_lib
// ./main
// 1 2 3 4 5 