//! Thread pool implementation for concurrent PDF generation.
//!
//! Supports single-threaded mode (`max_jobs=1`) for debugging and sequential processing,
//! or multi-threaded mode for parallel processing of multiple BOM files.
//!
//! When `max_jobs` is 0 or not set, the pool uses all available CPU cores for maximum parallelism.

use crate::lib_utils::concurrency::common::Job;
use crate::lib_utils::concurrency::worker::Worker;
use crate::lib_utils::errors::Vex2PdfError;
use log::debug;
use std::fmt::{Display, Formatter};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// TODO split this off into its own crate
pub(crate) struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
    num_threads: u8,
}

impl ThreadPool {
    /// Creates a new thread pool with the following behavior constraints:
    /// - pool_size is `0`: runs in multithreaded default mode using maximum parallelism
    /// - pool_size is `1`: runs in single-threaded mode (all jobs are run in the main thread)
    /// - pool_size is `1<N<=255`: runs in multithreaded mode with `N` jobs
    pub(crate) fn new(pool_size: u8) -> Self {
        if pool_size == 0 {
            Self::default()
        } else if pool_size == 1 {
            Self {
                workers: Vec::new(),
                sender: None,
                num_threads: pool_size,
            }
        } else {
            let (sender, receiver) = mpsc::channel::<Job>();

            let mut workers = Vec::with_capacity(pool_size as usize);

            let receiver = Arc::new(Mutex::new(receiver));

            for id in 1..=pool_size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            Self {
                workers,
                sender: Some(sender),
                num_threads: pool_size,
            }
        }
    }

    /// Executes a job on the thread pool.
    ///
    /// # Behavior
    /// - **Single-threaded mode** (`max_jobs=1`): Job executes synchronously in the calling thread
    /// - **Multi-threaded mode**: Job is queued and executed asynchronously by worker threads
    pub fn execute<F>(&self, f: F) -> Result<(), Vex2PdfError>
    where
        F: FnOnce() + Send + 'static,
    {
        if self.is_single_threaded() {
            f();
            Ok(())
        } else {
            self.sender
                .as_ref()
                .unwrap()
                .send(Box::new(f))
                .map_err(|e| e.into())
        }
    }

    /// Returns `true` if running in single-threaded mode.
    ///
    /// Single-threaded mode is active when `max_jobs=1`, resulting in:
    /// - No worker threads spawned
    /// - No message passing channel created
    /// - All jobs executed synchronously in the main thread
    pub fn is_single_threaded(&self) -> bool {
        self.sender.is_none() && self.workers.is_empty()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // drop the sender first which causes receivers to error out gracefully
        drop(self.sender.take());
        // now workers will error out thus unblocking their recv calls

        for worker in &mut self.workers {
            debug!("Shutting down worker {}", worker.id);
            worker.thread.take().unwrap().join().unwrap();
        }
    }
}

impl Default for ThreadPool {
    fn default() -> Self {
        let max_threads = thread::available_parallelism().map(|e| e.get()).expect("Unable to find any threads to run with. Possible system-side restrictions or limitations");

        // saturate to u8::MAX if number of threads is larger than what u8 can hold
        ThreadPool::new(u8::try_from(max_threads).unwrap_or(u8::MAX))
    }
}

impl Display for ThreadPool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_single_threaded() {
            write!(f,"Concurrency Disabled: running all jobs sequentially in main thread. A user override forced this through an VEX2PDF_MAX_JOBS or the --max-jobs cli argument")
        } else {
            write!(
                f,
                "Concurrency Enabled: running with {} jobs",
                self.num_threads
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    #[test]
    fn test_threadpool_creation_modes() {
        // Test pool with size 0 (default - max parallelism)
        let pool_default = ThreadPool::new(0);
        assert!(pool_default.num_threads > 0);
        assert!(!pool_default.is_single_threaded());

        // Test pool with size 1 (single-threaded)
        let pool_single = ThreadPool::new(1);
        assert_eq!(pool_single.num_threads, 1);
        assert!(pool_single.is_single_threaded());
        assert!(pool_single.workers.is_empty());
        assert!(pool_single.sender.is_none());

        // Test pool with size 4 (multi-threaded)
        let pool_multi = ThreadPool::new(4);
        assert_eq!(pool_multi.num_threads, 4);
        assert!(!pool_multi.is_single_threaded());
        assert_eq!(pool_multi.workers.len(), 4);
        assert!(pool_multi.sender.is_some());
    }

    #[test]
    fn test_single_threaded_execution() {
        let pool = ThreadPool::new(1);
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = Arc::clone(&counter);

        // Execute job synchronously
        pool.execute(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        })
        .expect("Failed to execute job");

        // In single-threaded mode, job executes immediately
        let value = *counter.lock().unwrap();
        assert_eq!(value, 1);
    }

    #[test]
    fn test_multi_threaded_execution() {
        let pool = ThreadPool::new(2);
        let results = Arc::new(Mutex::new(Vec::new()));

        // Execute multiple jobs
        for i in 0..5 {
            let results_clone = Arc::clone(&results);
            pool.execute(move || {
                std::thread::sleep(Duration::from_millis(10));
                results_clone.lock().unwrap().push(i);
            })
            .expect("Failed to execute job");
        }

        // Drop pool to wait for all jobs to complete
        drop(pool);

        // Verify all jobs completed
        let final_results = results.lock().unwrap();
        assert_eq!(final_results.len(), 5);
        // Results may be in any order due to concurrency
        for i in 0..5 {
            assert!(final_results.contains(&i));
        }
    }

    #[test]
    fn test_get_num_threads() {
        let pool1 = ThreadPool::new(1);
        assert_eq!(pool1.num_threads, 1);

        let pool4 = ThreadPool::new(4);
        assert_eq!(pool4.num_threads, 4);

        let pool_default = ThreadPool::default();
        assert!(pool_default.num_threads > 0);
    }

    #[test]
    fn test_is_single_threaded() {
        let pool_single = ThreadPool::new(1);
        assert!(pool_single.is_single_threaded());

        let pool_multi = ThreadPool::new(2);
        assert!(!pool_multi.is_single_threaded());

        let pool_default = ThreadPool::default();
        assert!(!pool_default.is_single_threaded());
    }

    #[test]
    fn test_pool_graceful_shutdown() {
        let pool = ThreadPool::new(3);
        let completed = Arc::new(Mutex::new(0));

        // Execute several jobs
        for _ in 0..10 {
            let completed_clone = Arc::clone(&completed);
            pool.execute(move || {
                std::thread::sleep(Duration::from_millis(20));
                *completed_clone.lock().unwrap() += 1;
            })
            .expect("Failed to execute job");
        }

        // Drop pool - should wait for all jobs to complete
        drop(pool);

        // All jobs should have completed
        assert_eq!(*completed.lock().unwrap(), 10);
    }
}
