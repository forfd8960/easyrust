use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kvserver::{CommandRequest, CommandResponse};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    println!("client");

    let addr = "127.0.0.1:9900";
    let stream = TcpStream::connect(addr).await?;

    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();

    let cmd = CommandRequest::new_hset("table1", "hello", "server".into());

    client.send(cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        println!("get response: {}", data);
    }

    Ok(())
}
