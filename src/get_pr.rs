
async fn read_pull_request_and_extract() -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    // Get GitHub token and other parameters from the environment
    let token = env::var("GITHUB_TOKEN")?;
    let pr_number: u32 = env::var("PR_NUMBER")?.parse()?;

    // Fetch the pull request content
    let url = format!(
        "https://api.github.com/repos/dericko681/fibbot/pulls/{}",
        env::var("GITHUB_REPOSITORY_OWNER")?,
        env::var("GITHUB_REPOSITORY_NAME")?,
        pr_number
    );

    let client = Client::new();
    let response: Value = client
        .get(&url)
        .header("User-Agent", "reqwest")
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await?;

    // Extract the body of the pull request
    let body = response["body"].as_str().unwrap_or("");

    // Call the extract function
    let extracted_numbers = extract_numbers(body);

    Ok(extracted_numbers)
}