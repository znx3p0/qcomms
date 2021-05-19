# QCOMMS

qcomms is a small library that offers a simple message passing trait.
it also offers keepalive and other stream helpers.

```rust
use qcomms::sync::SyncObjComms;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    hello: String,
    val: u32,
}

fn main() {
    std::thread::spawn(||{
        let listener = std::net::TcpListener::bind("127.0.0.1:3022").unwrap();
        let (mut stream, _) = listener.accept().unwrap();
        let message: Message = stream.rx().unwrap();
        println!("{:?}", message);
    });

    let m = Message {
        hello: "hello".to_string(),
        val: 12,
    };

    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut stream = std::net::TcpStream::connect("127.0.0.1:3022").unwrap();
    stream.tx(&m).unwrap();
}
```
