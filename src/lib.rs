use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>; // type alias, hold types of closures

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver)); // convert to smart pointer thread safe

        let mut workers = Vec::with_capacity(size); // mark as mutable, as it will changed

        for id in 0..size {
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver)
            ));
        }
        

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f); // create shared job through threads
        self.sender.send(job).unwrap(); // send job
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // move : for moving receiver to scope
        // loop : loop forever for receiving jobs
        let thread = thread::spawn(move || loop {
            let job = receiver
                .lock() // acquire mutex
                .unwrap() // handle if fail 
                .recv() // receive job
                .unwrap();
                
                println!("Worker {} received job!", id);

                job();
        });

        Worker { id, thread }
    }
}