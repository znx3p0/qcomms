# QCOMMS

qcomms is a small library that offers a simple message passing trait.
it also offers keepalive and other stream helpers.

```rust
use qcomms::SyncObjComms;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    hello: String,
    val: u32,
}

fn main() {
    std::thread::spawn(||{
        let listener = std::net::TcpListener::bind("127.0.0.1:3022").unwrap();
        let (mut stream, _) = listener.accept().unwrap();
        let msg: Message = stream.rx().unwrap();
        println!("message {:?}", msg);
    });

    let m = Message {
        hello: "test message".into(),
        val: 12,
    };
    std::thread::sleep(std::time::Duration::from_secs(1));
    let mut p = std::net::TcpStream::connect("127.0.0.1:3022").unwrap();
    p.tx(&m).unwrap();
}
```
