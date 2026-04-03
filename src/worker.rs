use std::{sync::Arc, thread};

use eject::device::Device;
use tokio::sync::{Mutex, broadcast::Receiver};

enum WorkerState {
    WAITING,
    RIP,
    TRANSCODE,
}

pub struct Worker {
    state: Arc<Mutex<WorkerState>>,
}

impl Worker {
    pub fn new(mut receiver: Receiver<Arc<str>>, eject_dev: Device) -> Self {
        //eject before entering ripping loop
        eject_dev.eject().unwrap();

        thread::spawn(async move || {
            loop {
                let p = receiver.recv().await.unwrap();
            }
        });

        return Worker {
            state: WorkerState::WAITING,
        };
    }

    pub fn get_status(&self) -> WorkerState {
        self.state.lock().await
    }
}
