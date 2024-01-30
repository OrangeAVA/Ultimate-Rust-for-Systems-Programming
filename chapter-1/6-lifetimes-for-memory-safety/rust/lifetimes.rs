fn longest_common_prefix<'a>(x: &'a str, y: &'a str) -> &'a str {
    let min_length = std::cmp::min(x.len(), y.len());
    let bytes_x = x.as_bytes();
    let bytes_y = y.as_bytes();

    for i in 0..min_length {
        if bytes_x[i] != bytes_y[i] {
            return &x[..i];
        }
    }

    &x[..min_length]
}

fn main() {
    let string1 = "abc";
    let result;
    {
        let string2 = "abdef";
        result = longest_common_prefix(string1, string2);
    }
    println!("The longest common prefix is: {}", result);
}

// rustc lifetimes.rs && ./lifetimes
// The longest common prefix is: ab