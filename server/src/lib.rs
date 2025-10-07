use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

/// A ThreadPool executes closures concurrently, up to the limit
/// imposed by the call to `ThreadPool::new`.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

/// A Job is a closure that can be sent between threads and has
/// static duration.
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The `size` is the number of threads in the pool.
    ///
    /// # Panics
    /// `new` will panic if the size is 0.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Execute a closure using one of the Workers in this ThreadPool,
    /// if available. Otherwise will hold the closure in a queue
    /// until a Worker is available.
    ///
    /// # Panics
    /// `execute` will panic if the [mpsc::Sender] used internally
    /// cannot send the closure to the Worker pool, such as if the
    /// ThreadPool has been shut down. However, this is currently
    /// only possible by dropping a ThreadPool.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}

/// A Worker will receive [Job]s from a [ThreadPool] and execute them
/// in a spawned thread.
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    /// Create a new Worker.
    ///
    /// A Worker is identified by an `id`, which can be non-unique
    /// and is only used for display output when a new [Job] arrives.
    /// Each Worker acquires a lock on the [mpsc::Receiver] from
    /// the [ThreadPool] via a [Mutex] wrapped in an [Arc].
    /// If the [ThreadPool] is dropped, each Worker will shut down
    /// gracefully.
    ///
    /// # Panics
    /// If another thread has poisoned the [Mutex] lock on the
    /// [mpsc::Receiver] by panicking without releasing the lock,
    /// the next thread to try to acquire a lock will panic.
    ///
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job, executing");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected, shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}
