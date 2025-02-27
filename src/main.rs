use std::env;

use fibonacci::fibonacci_up_to;
use get_pr::read_pull_request_and_extract;
mod fibonacci; mod extract; mod get_pr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let extracted_numbers = read_pull_request_and_extract().await?;

    for number in extracted_numbers {
        let fib_sequence = fibonacci_up_to(number);
        println!("Extracted Number: {}, Fibonacci Sequence: {:?}", number, fib_sequence);
    }

    Ok(())

    
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

