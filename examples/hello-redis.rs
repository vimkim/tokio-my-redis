use mini_redis::client;
use mini_redis::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    let mut client = client::connect("127.0.0.1:6379").await?;
    println!("Hello, world!2");

    client.set("hello", "world".into()).await?;
    println!("Hello, world!3");

    let result = client.get("hello").await?;
    println!("Hello, world!4");

    println!("got value from the server; result={:?}", result);

    Ok(())
}
