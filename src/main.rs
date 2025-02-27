use std::env;
use dotenv::dotenv;

use extract::extract_numbers;
use fibonacci::fibonacci_up_to;
use get_pr::fetch_pull_request_body;
use post_comment::post_comment;
mod extract;
mod fibonacci;
mod get_pr; mod post_comment;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // dotenv().ok();
    // Get parameters from environment variables
    let pr_number: u64 = env::var("GITHUB_PULL_REQUEST_NUMBER")?.parse()?;
    let enable_fibonacci: bool = env::var("ENABLE_FIBONACCI")?.parse().unwrap_or(false);
    let threshold: u32 = env::var("THRESHOLD").unwrap_or_else(|_| "100".to_string()).parse().unwrap_or(100);

    // Fetch the pull request body
    let body = fetch_pull_request_body(pr_number)?;

    // Extract numbers from the body
    let numbers = extract_numbers(body.clone());

    // Prepare the comment
    let mut comment = String::new();

    if enable_fibonacci {
        let mut fib_results = Vec::new();
        for &number in &numbers {
            if number <= threshold {
                fib_results.extend(fibonacci_up_to(number));
            }
        }
        comment = format!(
            "Fibonacci numbers for extracted values: {:?}",
            fib_results
        );
    } else {
        comment = format!("Extracted numbers: {:?}", numbers);
    }

    // Post the comment
    post_comment(pr_number, &comment)?;

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

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Get parameters from environment variables

//     // let enable_fibonacci: bool = env::var("ENABLE_FIBONACCI")?.parse().unwrap_or(false);
//     // let threshold: u32 = env::var("THRESHOLD").unwrap_or_else(|_| "100".to_string()).parse().unwrap_or(100);

//     // Fetch the pull request body
   

//     // Extract numbers from the body
//     let numbers = extract_numbers(body.clone());

//     // Prepare the comment
//     let mut comment = String::new();

//     if enable_fibonacci {
//         let mut fib_results = Vec::new();
//         for &number in &numbers {
//             if number <= threshold {
//                 fib_results.extend(fibonacci_up_to(number));
//             }
//         }
//         comment = format!(
//             "Fibonacci numbers for extracted values: {:?}",
//             fib_results
//         );
//     } else {
//         comment = format!("Extracted numbers: {:?}", numbers);
//     }

//     // Post the comment
//     post_comment(pr_number, &comment)?;

//     Ok(())
// }
