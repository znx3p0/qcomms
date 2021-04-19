use std::io::Result;

use qcomms::ObjComms;
use serde::{Deserialize, Serialize};
use tokio::{
    net::{TcpListener, TcpStream},
    spawn,
};

#[tokio::test]
async fn test() -> Result<()> {
    let handle = spawn(async {
        server_start().await.unwrap();
    });

    spawn(async {
        client_start().await.unwrap();
    });

    handle.await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    hello: String,
    val: u32,
}

pub async fn server_start() -> Result<()> {
    let l = TcpListener::bind("127.0.0.1:9091").await?;

    let s = l.accept().await?;

    let mut stream = s.0;
    let d: String = stream.rx().await?;
    println!("message: {:?}", d);
    let d: Message = stream.rx().await?;
    println!("message: {:?}", d);
    Ok(())
}

pub async fn client_start() -> Result<()> {
    let mut conn = TcpStream::connect("127.0.0.1:9091").await?;

    conn.tx("Hi there!").await?;

    let p = Message {
        hello: "world".into(),
        val: 23924,
    };
    conn.tx(&p).await?;

    Ok(())
}
