use mini_redis::{client, Result};

// mache erstmal die docs durch also noch nicht direkt ein messenger

#[tokio::main]
async fn main() -> Result<()> {
    // connecting to the redis
    let mut client = client::connect("127.0.0.1:6379").await?;

    // set a key named "hello" to value "world"
    client.set("hello", "world".into()).await?;

    // get the value of the key "hello again"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}