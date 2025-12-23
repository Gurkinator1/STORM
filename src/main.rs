use std::thread;

mod udev;
mod worker;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    //init udev monitor
    let mon = crate::udev::monitor().unwrap();

    let mut handles = Vec::new();
    for n in 0..5 {
        let mut rx = mon.subscribe();
        handles.push(thread::spawn(async move || {
            let msg = rx.recv().await.unwrap();
            println!("worker {} recvd: {}", n, msg);
        }));
    }

    println!("ready!");

    for handle in handles {
        handle.join().unwrap().await;
    }

    Ok(())
}
