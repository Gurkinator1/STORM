use std::sync::Arc;

use eject::device::{Device, DriveStatus};
use tokio::sync::{broadcast::Receiver, watch};

use crate::{makemkv::MakeMKV, udev::UdevEvent};

#[derive(Clone, Copy, serde::Serialize, PartialEq, Eq)]
pub enum WorkerState {
    Idle,
    Ripping,
    Transcoding,
    Error,
}

enum WorkerEvent {
    Udev(Arc<UdevEvent>),
    MkvDone,
}

pub struct Worker {
     state_rx: watch::Receiver<WorkerState>,
}
impl Worker {
    pub fn new(mut receiver: Receiver<Arc<UdevEvent>>, drive: Device, drive_path: &str) -> Self {
        let (tx, rx) = watch::channel(WorkerState::Idle);
        let drive_path = drive_path.to_owned();
        
        //eject before entering ripping loop
        drive.eject().unwrap();
        
        tokio::spawn(async move {
            let mut current_state = WorkerState::Idle;
            while let Ok(event) = receiver.recv().await {
                
                //check if event is relevant for this worker
                if event.dev!=drive_path {
                    continue;
                }

                current_state = step(current_state, &drive).await;
                let _ = tx.send(current_state);
            }
        });

        return Worker {
            state_rx: rx
        };
    }

    pub async fn get_status(&self) -> WorkerState {
        *self.state_rx.borrow()
    }
}

async fn step(state: WorkerState, drive: &Device) -> WorkerState {
    match state {
        WorkerState::Idle => {
            match drive.status() {
                Ok(DriveStatus::Loaded) => {
                    if let Ok(mkv) = MakeMKV::new(&None).await{
                        WorkerState::Ripping
                    }
                    else {
                        WorkerState::Error
                    }
                }
                _ => WorkerState::Idle,
            }
        }

        WorkerState::Ripping => {
            // check progress, completion, etc.
            state
        }

        WorkerState::Transcoding => {
            state
        }

        WorkerState::Error => state,
    }
}