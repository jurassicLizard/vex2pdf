//! Worker model for concurrent jobs handling
use crate::lib_utils::concurrency::common::Job;
use log::debug;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub(crate) struct Worker {
    pub(super) id: u8,
    pub(super) thread: Option<JoinHandle<()>>,
}

impl Worker {
    /// Creates a new worker that spawns a thread to process jobs from the shared receiver.
    ///
    /// The worker continuously receives jobs from the channel until the sender is dropped,
    /// at which point it exits gracefully.
    pub(crate) fn new(id: u8, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn test_worker_creation() {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let worker = Worker::new(1, Arc::clone(&receiver));

        assert_eq!(worker.id, 1);
        assert!(worker.thread.is_some());

        // Clean up
        drop(sender);
        worker.thread.unwrap().join().unwrap();
    }

    #[test]
    fn test_worker_executes_job() {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let executed = Arc::new(Mutex::new(false));
        let executed_clone = Arc::clone(&executed);

        let worker = Worker::new(2, Arc::clone(&receiver));

        // Send a job
        sender
            .send(Box::new(move || {
                *executed_clone.lock().unwrap() = true;
            }))
            .unwrap();

        // Give worker time to execute
        std::thread::sleep(Duration::from_millis(50));

        // Verify job was executed
        assert!(*executed.lock().unwrap());

        // Clean up
        drop(sender);
        worker.thread.unwrap().join().unwrap();
    }

    #[test]
    fn test_worker_shutdown_on_channel_close() {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let worker = Worker::new(3, Arc::clone(&receiver));

        // Close channel by dropping sender
        drop(sender);

        // Worker thread should exit gracefully
        let result = worker.thread.unwrap().join();
        assert!(result.is_ok());
    }
}
