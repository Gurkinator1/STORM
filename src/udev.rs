use std::{sync::Arc, thread};

use anyhow::{Ok, Result};
use tokio::sync::broadcast::{self, Receiver};
use udev::{
     EventType, MonitorBuilder, mio::{Events, Interest, Poll, Token}
};

#[derive(Debug)]
pub struct UdevEvent {
    pub dev: String,
    pub event_type: EventType,
}

pub fn monitor() -> anyhow::Result<Receiver<Arc<UdevEvent>>> {
    let (tx, rx) = broadcast::channel::<Arc<UdevEvent>>(16);

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
                    socket.iter().for_each(|event| {
                        let event_type = event.event_type();
                        let dev = event.devnode().unwrap().to_string_lossy().to_string();
                        ltx.send(Arc::from(UdevEvent {dev, event_type})).unwrap();
                    });
                }
            }
        }
    });

    return Ok(rx);
}
