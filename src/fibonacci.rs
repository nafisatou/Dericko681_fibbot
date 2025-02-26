pub fn fibonacci_up_to(max: u32) -> Vec<u32> {
    let mut fib_sequence = vec![1, 1];
    //let mut fib_sequence = Vec::new();
    while let Some(&last) = fib_sequence.last() {
        let next = fib_sequence[fib_sequence.len() - 1] + fib_sequence[fib_sequence.len() - 2];
        if max == 0 {
            fib_sequence = vec![];
            break;
        }
        if next > max {
            break;
        }
        fib_sequence.push(next);
    }
    fib_sequence.into_iter().collect() // Skip the first two (0, 1)
}

#[test]
fn test_fibonacci_up_to() {
    let result = fibonacci_up_to(10);
    assert_eq!(result, vec![1, 1, 2, 3, 5, 8]);
}
#[test]
fn test_fibonacci_up_to1() {
    let result = fibonacci_up_to(1);
    assert_eq!(result, vec![1, 1]);
}
// #[test]
// fn test_fibonacci_up_to2() {
//     let result:Vec<u32> = fibonacci_up_to(0);
//     assert!(result == vec![]);
// }
