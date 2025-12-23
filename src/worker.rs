enum WorkerState {
    WAITING,
    RIP,
    TRANSCODE,
}

struct Worker {
    state: WorkerState,
}

impl Worker {
    pub fn new() -> Worker {
        return Worker {
            state: WorkerState::WAITING,
        };
    }
}
