 pub fn extract_numbers(content: &str) -> Vec<u32> {
    content
        .split_whitespace()
        .filter_map(|word| word.trim_matches(|c: char| !c.is_digit(10)).parse::<u32>().ok())
        .filter(|&num| num > 0) // Ensure we only collect positive numbers
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numbers() {
        let content = "Here are some numbers: 1, 42 and 100.";
        let numbers = extract_numbers(content);
        assert_eq!(numbers, vec![1, 42, 100]);
    }
}
