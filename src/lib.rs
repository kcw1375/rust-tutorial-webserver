use std::thread;
use std::sync::{mpsc, Arc, Mutex};

// a Job is an arbitrary function to perform
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job), // holds a job for thread to execute
    Terminate, // signals thread to stop
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}


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

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) { 
        // clean up the threads and finish their work for graceful shutdown
        println!("Sending terminate message to all workers.");

        // we cannot guarantee that two send calls in one loop get received by the same thread
        // thus we need two loops to prevent deadlock

        // first need to loop to terminate all workers
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        // then need to loop to perform final work
        for worker in &mut self.workers {
            println!("Shtting down worker {}", worker.id);

            // move thread out of worker so that we can call join, which takes ownership
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // make the thread keep waiting for a job and then run it
        let thread = thread::spawn(move || loop {
            // the mutex lock ensures that only one worker thread at a time tries to request a job
            // mutex lock is released at the end of the let statement, 
            // allowing next thread to request job while this job executes
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {id} got a job, executing.");
                    job();
                },
                Message::Terminate => {
                    println!("Worker {id} told to terminate.");
                    break;
                }
            }
            
        });

        Worker {
            id, 
            thread: Some(thread)
        }
    }
}