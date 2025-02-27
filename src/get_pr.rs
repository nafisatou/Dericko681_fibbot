use reqwest::blocking::get;
use std::env;

pub fn fetch_pull_request_body(pr_number: u64) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/pulls/{}",
        env::var("GITHUB_REPOSITORY_OWNER")?,
        env::var("GITHUB_REPOSITORY_NAME")?,
        pr_number
    );

    let response = get(&url)?.text()?;
    Ok(response)
}