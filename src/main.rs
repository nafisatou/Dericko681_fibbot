mod extract;
mod fibonacci;
mod get_pr;
mod post_comment;
use extract::extract_numbers;
use fibonacci::fibonacci_up_to;
use get_pr::get_pr_body;
use post_comment::post_comment;
use std::env;

fn main() {
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

    // Example values
    // let owner = "dericko681";
    // let repo = "fibbot";
    // let pr_number: u32 = 4;

    match get_pr_body(pr_number, &owner, &repo) {
        Ok(content) => {
            let extracted_numbers = extract_numbers(content);
            // Further processing...
            println!("Extracted numbers: {:?}", extracted_numbers);
            for number in extracted_numbers {
                let fibonacci_results = fibonacci_up_to(number);
                let comment_body = format!(
                    "Fibonacci numbers up to {}: {:?}",
                    number, fibonacci_results
                );
                println!("the comments {}", comment_body);
                //    println!("The fibonacci of {} is: {:?}", number,fibonacci_up_to(number));
                match post_comment(pr_number, &owner, &repo, comment_body) {
                    Ok(_) => println!("Comment posted successfully!"),
                    Err(e) => eprintln!("Error posting comment: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error fetching PR body: {}", e),
    }
}
