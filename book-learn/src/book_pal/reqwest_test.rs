#[tokio::main]
pub async fn request_test() -> Result<(), Box<dyn std::error::Error> > {
    let body = reqwest::get("https://www.rust-lang.org/")
        .await ?
        .text()
        .await ? ;
    println!("body: {}", body);
    Ok( () )
}