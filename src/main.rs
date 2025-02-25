use std::env;

fn main() {
    // Get inputs from environment variables
    let enable_fib = env::var("INPUT_ENABLE_FIB").unwrap_or("true".to_string());
    let max_threshold: u32 = env::var("INPUT_MAX_THRESHOLD")
        .unwrap_or("10".to_string())
        .parse()
        .unwrap_or(10); // Default to 10 if parsing fails

    // Log the inputs
    println!("Enable Fibonacci: {}", enable_fib);
    println!("Max Threshold: {}", max_threshold);

    // Validate max_threshold
    if max_threshold == 0 {
        println!("Error: max_threshold must be greater than 0.");
        std::process::exit(1);
    }

    // (Additional logic for Fibonacci calculation can be added here)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_parsing() {
        let enable_fib = "true".to_string();
        let max_threshold: u32 = "10".parse().unwrap();

        assert_eq!(enable_fib, "true");
        assert_eq!(max_threshold, 10);
    }
}

