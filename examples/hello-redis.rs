use mini_redis::client;
use mini_redis::Result;
use tracing::info;
use tracing::instrument;

#[instrument]
fn my_function(arg: i32) {
    info!("Inside my_function");
    another_function();
}

#[instrument]
fn another_function() {
    info!("Inside another_function");
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_line_number(true)
        .with_target(true)
        .init();

    my_function(5);

    println!("Hello, world!");
    let mut client = client::connect("127.0.0.1:6379").await?;
    println!("Hello, world!2");

    client.set("hello", "world".into()).await?;
    println!("Hello, world!3");

    my_function(5);
    let result = client.get("hello").await?;
    println!("Hello, world!4");

    println!("got value from the server; result={:?}", result);

    Ok(())
}
