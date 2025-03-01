// use reqwest;
// use serde_json::Value;

// pub async fn get_pr_body(
//        repo: &str,
//     pr_number: u32,
//     file_path: &str,
// ) -> Result<String, Box<dyn std::error::Error>> {
//     let url = format!("https://api.github.com/repos/{}/pulls/{}/files/{}",repo, pr_number, file_path);
//     let response = reqwest::get(url).await?;

//     if response.status().is_success() {
//         let json: Value = response.json().await?;
//         let content = json["content"].as_str().ok_or("File content not found")?;
//         let decoded_content = base64::decode(content).unwrap();
//         let file_content = String::from_utf8(decoded_content).unwrap();
//         Ok(file_content)
//     } else {
//         Err(format!("Failed to retrieve PR {}: {}", pr_number, response.status()).into())
//     }
// }



use anyhow::{Result, Context};
use reqwest::Client;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct PullRequest {
    body: Option<String>,
}

#[derive(Deserialize)]
struct DiffEntry {
    filename: String,
    patch: Option<String>,
}


pub async fn get_pr_body(pr_number: u32, repo: &str, token:&str) -> Result<String> {
   
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/pulls/{}/files",
        repo, pr_number
    );

    let response = client
        .get(&url)
        .header("User-Agent", "FibBot")
        .bearer_auth(token)
        .send()
        .await
        .context("Failed to send request to GitHub API")?;

    let response_text = response.text().await.context("Failed to read response text")?;
    println!("PR Files fetched successfully");

    let files: Vec<DiffEntry> = serde_json::from_str(&response_text).context("Failed to parse PR files")?;
    let content = files.iter()
        .filter_map(|file| file.patch.as_ref())
        .cloned()
        .collect::<Vec<String>>()
        .join("\n");
    
    Ok(content)  // Returns the accumulated string as a Result<String>
}
