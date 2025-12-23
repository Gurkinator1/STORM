use std::{sync::Arc, thread};

use anyhow::{Ok, Result};
use tokio::sync::broadcast::{self, Receiver};
use udev::{
    MonitorBuilder,
    mio::{Events, Interest, Poll, Token},
};

type Message = Arc<str>;
pub fn monitor() -> anyhow::Result<Receiver<Message>> {
    let (tx, rx) = broadcast::channel::<Message>(16);

    let ltx = tx.clone();
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
                        let str = x.devpath().to_str().unwrap();
                        println!("{}", &str);
                        ltx.send(Arc::from(str)).unwrap();
                    });
                }
            }
        }
    });

    return Ok(rx);
}
