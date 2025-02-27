use reqwest::blocking::Client;
use std::env;

pub fn get_pr_body(pr_number: u32, owner: &str, repo: &str) -> Result<String, Box<dyn std::error::Error>> {
    let token = env::var("GITHUB_TOKEN")?; 
    let url = format!("https://api.github.com/repos/{}/{}/pulls/{}", owner, repo, pr_number);

    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "rust-github-action")
        .bearer_auth(token)
        .send()?;

    if response.status().is_success() {
        let json: serde_json::Value = response.json()?;
        if let Some(body) = json.get("body") {
            return Ok(body.as_str().unwrap_or("").to_string());
        }
    }

    Err("Failed to get PR body".into())
}