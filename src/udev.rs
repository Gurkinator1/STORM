use std::thread;

use anyhow::{Ok, Result};
use spmc::Receiver;
use udev::{
    MonitorBuilder,
    mio::{Events, Interest, Poll, Token},
};

pub fn monitor() -> anyhow::Result<Receiver<u8>> {
    let (mut tx, rx) = spmc::channel::<u8>();
    tx.send(5).unwrap();

    thread::spawn(move || -> Result<()> {
        let mut socket = MonitorBuilder::new()?
            .match_subsystem_devtype("block", "disk")?
            .listen()?;
        let mut poll = Poll::new()?;
        poll.registry().register(
            &mut socket,
            Token(0),
            Interest::READABLE | Interest::WRITABLE,
        )?;
        let mut events = Events::with_capacity(1024);

        loop {
            poll.poll(&mut events, None)?;
            for event in &events {
                if event.token() == Token(0) && event.is_writable() {
                    socket.iter().for_each(|x| {
                        println!("{}", x.devpath().to_str().unwrap());
                        tx.send(5).unwrap();
                    });
                }
            }
        }
    });

    return Ok(rx);
}
