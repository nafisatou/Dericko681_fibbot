mod extract; mod fibonacci; mod get_pr; mod post_comment;
use fibonacci::fibonacci_up_to;
use post_comment::post_comment;
use get_pr::get_pr_body;
use extract::extract_numbers;

fn main() {
    // Example values
    let owner = "dericko681";
    let repo = "fibbot";
    let pr_number: u32 = 1; 

    match get_pr_body(pr_number, owner, repo) {
        Ok(content) => {
            let extracted_numbers = extract_numbers(content);
            // Further processing...
            println!("Extracted numbers: {:?}", extracted_numbers);
            for number in extracted_numbers{

            //    println!("The fibonacci of {} is: {:?}", number,fibonacci_up_to(number));
            post_comment(pr_number, owner, repo, fibonacci_up_to(number));
            }

        }
        Err(e) => eprintln!("Error fetching PR body: {}", e),
    }
}
