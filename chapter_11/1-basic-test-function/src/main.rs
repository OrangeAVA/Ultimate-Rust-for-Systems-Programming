#[test]
fn test_complex_addition() {
    let input_1 = 42;
    let input_2 = 58;

    let result = perform_complex_addition(input_1, input_2);

    assert_eq!(result, 100, "The addition result is incorrect.");
}

fn perform_complex_addition(a: i32, b: i32) -> i32 {
    a + b
}
