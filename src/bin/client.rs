use mini_redis::client;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    color_backtrace::install();
    client::connect("127.0.0.1:6379").await.unwrap();

    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await.unwrap();
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
}
