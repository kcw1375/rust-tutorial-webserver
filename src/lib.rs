use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// a Job is an arbitrary function to perform
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// The 'new' function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        // ThreadPool will hold onto the sending side of the channel
        // receiver must be made into an Arc<Mutex> so that all threads can use one receiver
        // (only can have a single consumer)
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // make the thread keep waiting for a job and then run it
        let thread = thread::spawn(move || loop {
            // the mutex lock ensures that only one worker thread at a time tries to request a job
            // mutex lock is released at the end of the let statement, 
            // allowing next thread to request job while this job executes
            let job = receiver.lock().unwrap().recv().unwrap();
            
            println!("Worker {id} got a job, executing.");

            job();
        });

        Worker {id, thread}
    }
}