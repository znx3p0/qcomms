# QCOMMS

qcomms is a small library that offers a simple message passing trait.
it also offers keepalive and other stream helpers.

```rust
use qcomms::ObjComms;
use serde::{Serialize, Deserialize};
use async_std::task;
use async_std::task::sleep;
use async_std::net::{TcpListener, TcpStream};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    hello: String,
    val: u32,
}

#[async_std::main]
async fn main() {
    task::spawn(async move {
        let listener = TcpListener::bind("127.0.0.1:3022").await.unwrap();
        let (mut stream, _) = listener.accept().unwrap();
        let message: Message = stream.rx().await.unwrap();
        println!("{:?}", message);
    });

    let m = Message {
        hello: "hello".to_string(),
        val: 12,
    };

    task::sleep(Duration::from_secs(1)).await;
    let mut stream = TcpStream::connect("127.0.0.1:3022").await.unwrap();
    stream.tx(&m).await.unwrap();
}
```
