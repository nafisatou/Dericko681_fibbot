use reqwest::blocking::{get, Client};
use std::env;
pub fn post_comment(pr_number: u64, comment: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues/{}/comments",
        env::var("GITHUB_REPOSITORY_OWNER")?,
        env::var("GITHUB_REPOSITORY_NAME")?,
        pr_number
    );

    let client = Client::new();
    let payload = serde_json::json!({ "body": comment });

    client
        .post(&url)
        .header("Authorization", format!("token {}", env::var("GITHUB_TOKEN")?))
        .json(&payload)
        .send()?
        .error_for_status()?;
    
    Ok(())
}