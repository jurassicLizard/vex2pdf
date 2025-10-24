//! Worker model for concurrent jobs handling
use crate::lib_utils::concurrency::common::Job;
use log::debug;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub(crate) struct Worker {
    pub(super) id: usize,
    pub(super) thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub(crate) fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = std::thread::spawn(move || loop {
            // FIXME modify this to handle errors and push them to the joinhandle
            let job_msg = receiver.lock().unwrap().recv();

            match job_msg {
                Ok(job) => {
                    debug!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    debug!("Worker {id} disconnected; shutting down;");
                    break;
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}
