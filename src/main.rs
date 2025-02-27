mod extract; mod fibonacci; mod get_pr;
use fibonacci::fibonacci_up_to;
use get_pr::get_pr_body;
use extract::extract_numbers;

fn main() {
    // Example values
    let owner = "dericko681";
    let repo = "fibbot";
    let pr_number: u32 = 1; 

    match get_pr_body(pr_number, owner, repo) {
        Ok(content) => {
            let numbers = extract_numbers(content);
            // Further processing...
            println!("Extracted numbers: {:?}", numbers);
        }
        Err(e) => eprintln!("Error fetching PR body: {}", e),
    }
}
