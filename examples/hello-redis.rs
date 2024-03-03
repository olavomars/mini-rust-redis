use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("this is a key", "this is a value".into()).await?;

    let result = client.get("this is a key").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}    