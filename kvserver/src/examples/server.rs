use anyhow::{Ok, Result};
use async_prost::AsyncProstStream;
use kvserver::{CommandRequest, CommandResponse};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:9900";
    let listner = TcpListener::bind(addr).await?;
    println!("start listening on {}", addr);

    loop {
        let (stream, addr) = listner.accept().await?;
        println!("Client {:?} connected", addr);
        tokio::spawn(async move {
            let mut async_stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();

            while let Some(Ok(data)) = async_stream.next().await {
                println!("get command: {}", data);

                let mut resp = CommandResponse::default();
                resp.status = 404;
                resp.message = "Not found".to_string();
                async_stream.send(resp).await.unwrap();
            }

            println!("client {:?} disconnected", addr);
        });
    }
}
