use std::{thread, time::Duration};

mod udev;
mod worker;

fn main() {
    println!("Hello, world!");

    //init udev monitor
    let mon = crate::udev::monitor().unwrap();

    let mut handles = Vec::new();
    for n in 0..5 {
        let rx = mon.clone();
        handles.push(thread::spawn(move || {
            let msg = rx.recv().unwrap();
            println!("worker {} recvd: {}", n, msg);
        }));
    }

    thread::sleep(Duration::from_secs(60));

    for handle in handles {
        handle.join().unwrap();
    }
}
