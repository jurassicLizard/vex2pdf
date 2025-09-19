use crate::lib_utils::concurrency::common::Job;
use crate::lib_utils::concurrency::worker::Worker;
use crate::lib_utils::errors::Vex2PdfError;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub(crate) struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    pub(crate) fn new(pool_size: usize) -> Self {
        assert_ne!(pool_size, 0);

        let (sender, receiver) = mpsc::channel::<Job>();

        let mut workers = Vec::with_capacity(pool_size);

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 1..=pool_size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F) -> Result<(), Vex2PdfError>
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender
            .as_ref()
            .unwrap()
            .send(Box::new(f))
            .map_err(|e| e.into())
    }
}

impl Default for ThreadPool {
    fn default() -> Self {
        let max_threads = thread::available_parallelism().map(|e| e.get()).expect("Unable to find any threads to run with. Possible system-side restrictions or limitations");

        ThreadPool::new(max_threads)
    }
}
