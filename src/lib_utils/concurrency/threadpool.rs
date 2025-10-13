use crate::lib_utils::concurrency::common::Job;
use crate::lib_utils::concurrency::worker::Worker;
use crate::lib_utils::errors::Vex2PdfError;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// TODO split this off into its own crate
pub(crate) struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
    num_threads: usize
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
            num_threads: pool_size
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

    /// returns the active number of threads for the pool
    #[inline]
    pub fn get_num_threads(&self) -> usize { self.num_threads }


}

impl Drop for ThreadPool {
    fn drop(&mut self) {
       // drop the sender first which causes receivers to error out gracefully
        drop(self.sender.take());
        // now workers will error out thus unblocking their recv calls

        for worker in &mut self.workers {
            println!("Shutting down {}", worker.id);
            worker.thread.take().unwrap().join().unwrap();
        }
    }
}

impl Default for ThreadPool {
    fn default() -> Self {
        let max_threads = thread::available_parallelism().map(|e| e.get()).expect("Unable to find any threads to run with. Possible system-side restrictions or limitations");

        ThreadPool::new(max_threads)
    }
}
