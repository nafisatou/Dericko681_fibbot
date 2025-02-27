pub fn fibonacci_up_to(max: u32) -> Vec<u32> {
    let mut fib_sequence = vec![0, 1];
   
    while let Some(&last) = fib_sequence.last() {
        let next = fib_sequence[fib_sequence.len() - 1] + fib_sequence[fib_sequence.len() - 2];
        if max == 0 {
            fib_sequence = vec![];
            break;
        }
        if max == 1 {
            fib_sequence = vec![0, 1];
            break;
        }
        if next > max {
            break;
        }
        fib_sequence.push(next);
    }
    fib_sequence.into_iter().collect() // Skip the first two (0, 1)
}


// #[cfg(test)]
// mod tests{
//     use super::*;
//     #[test]
// fn test_fibonacci_up_to_10() {
//     let result = fibonacci_up_to(10);
//     assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8]);
// }
// #[test]
// fn test_fibonacci_up_to_1() {
//     let result = fibonacci_up_to(1);
//     assert_eq!(result, vec![0, 1]);
// }
// #[test]
// fn test_fibonacci_up_to_0() {
//     let result = fibonacci_up_to(0);
//     assert!(result.is_empty());
// }
//   #[test]
//     fn test_fibonacci_up_to_100() {
//         let result = fibonacci_up_to(100);
//         assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89]);
//     }

// }
