use std::{sync::{Arc, Mutex, mpsc::{self}}, thread};

pub struct ThreadPool{
    workers: Vec<Worker>,
    senders: Option<mpsc::Sender<Job>>
}



type Job = Box<dyn FnOnce() + Send + 'static>;



impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);
        
        let (senders, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool {workers, senders: Some(senders)}
    }
    
     pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.senders.as_ref().unwrap().send(job).unwrap();
    }
        
}
    
    
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
        
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.senders.take());

        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker.thread.join().unwrap();
        }
    }
}