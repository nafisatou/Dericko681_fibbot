mod extract;
mod fibonacci;
mod get_pr;
mod post_comment;
use extract::extract_numbers;
use fibonacci::fibonacci_up_to;
use get_pr::get_pr_body;
use post_comment::post_comment;
use std::env;

// fn main() {
//     // 2 3 5 hello
    //     let pr_number: u32 = env::var("PR_NUMBER")
    //     .expect("GITHUB_EVENT_NUMBER not set")
    //     .parse()
    //     .expect("GITHUB_EVENT_NUMBER is not a valid number");
    // let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    // // let pr_number = 6;
    // let owner = env::var("GITHUB_REPOSITORY_OWNER").expect("GITHUB_REPOSITORY_OWNER not set");

    // println!("pr_number: {}", pr_number);
    // println!("repo: {}", repo);
    // println!("owner: {}", owner);

//     // Example values
//     // let owner = "dericko681";
//     // let repo = "fibbot";
//     // let pr_number: u32 = 4;

//     match get_pr_body(pr_number, &owner, &repo) {
//         Ok(content) => {
//             let extracted_numbers = extract_numbers(content);
//             // Further processing...
//             println!("Extracted numbers: {:?}", extracted_numbers);
//             for number in extracted_numbers {
//                 let fibonacci_results = fibonacci_up_to(number);
//                 let comment_body = format!(
//                     "Fibonacci numbers up to {}: {:?}",
//                     number, fibonacci_results
//                 );
//                 println!("the comments {}", comment_body);
//                 //    println!("The fibonacci of {} is: {:?}", number,fibonacci_up_to(number));
//                 match post_comment(pr_number, &owner, &repo, comment_body) {
//                     Ok(_) => println!("Comment posted successfully!"),
//                     Err(e) => eprintln!("Error posting comment: {}", e),
//                 }
//             }
//         }
//         Err(e) => eprintln!("Error fetching PR body: {}", e),
//     }
// }

use tokio; // Ensure you have tokio as a dependency in your Cargo.toml

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Retrieve environment variables set in the GitHub Action
    // let pr_number: u32 = env::var("PR_NUMBER")?.parse()?;
    // let repo = env::var("GITHUB_REPOSITORY")?;
    let token = env::var("GITHUB_TOKEN")?;
    let enable_fib: bool = env::var("ENABLE_FIB")?.parse()?;
    let threshold: u32 = env::var("THRESHOLD")?.parse()?;
            let pr_number: u32 = env::var("PR_NUMBER")
        .expect("GITHUB_EVENT_NUMBER not set")
        .parse()
        .expect("GITHUB_EVENT_NUMBER is not a valid number");
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    // let pr_number = 6;
    let owner = env::var("GITHUB_REPOSITORY_OWNER").expect("GITHUB_REPOSITORY_OWNER not set");

    println!("pr_number: {}", pr_number);
    println!("repo: {}", repo);
    println!("owner: {}", owner);

    let pr_body = get_pr_body(pr_number, &repo, &token).await?;

    println!("pr body {:?}", pr_body);
    let numbers = extract_numbers(pr_body);
    println!("extracted numbers are {:?}", numbers);
    let mut comment = String::new();

    if enable_fib {
        // Calculate Fibonacci numbers up to the threshold
        let fib_numbers = fibonacci_up_to(threshold);

        // Filter numbers to only those that are in the Fibonacci series
        let fib_results: Vec<u32> = numbers
            .iter()
            .filter(|&&n| fib_numbers.contains(&n))
            .cloned()
            .collect();

        // Generate a comment
        comment.push_str("Fibonacci numbers found in the PR content:\n");
        for num in fib_results {
            comment.push_str(&format!("{}\n", num));
        }
    } else {
        comment.push_str("Fibonacci calculation is disabled.\n");
    }
println!("The comment is {}", comment);
    // Post the comment to the pull request
    post_comment(pr_number, &repo.split('/').next().unwrap(), &repo, comment);

    Ok(())
}
