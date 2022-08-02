// ch_01/minimal_reqwest/main.rs
// https://github.com/Rust-Web-Development/code/tree/main/ch_01/minimal_reqwest

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?;

    println!("{:?}", resp);
    Ok(())
}
