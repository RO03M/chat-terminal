use std::{sync::Arc, time::Duration};

use futures_util::StreamExt;
use tokio::sync::Mutex;
use tokio_tungstenite::connect_async;

#[derive(Debug)]
pub struct Foo {
    counter: String,
}

#[tokio::main]
async fn main() {
    let (ws_stream, response) = connect_async("ws://localhost:8080/chat")
        .await
        .expect("Failed to connect");

    let (_write, read) = ws_stream.split();

    println!("{}", response.status());

    let num = Arc::new(Mutex::new(Foo {
        counter: "init".into(),
    }));
    let num_cloned = num.clone();

    let read_future = read.for_each(|message| async {
        let data = message.unwrap().into_data();

        // println!("{:?}", String::from_utf8(data));
        let mut lock = num.lock().await;

        lock.counter = String::from_utf8(data).unwrap();
        println!("above {:?}", *lock);
    });

    let foo = tokio::spawn(async move {
        loop {
            let lock = num_cloned.lock().await;
            println!("bellow {:?}", *lock);
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    read_future.await;
    foo.await;
}
