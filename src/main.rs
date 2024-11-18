#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    basic().await?;

    Ok(())
}

async fn basic() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;
    println!("body = {:?}", body);

    Ok(())
}
