use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use crate::utils::Job;

pub struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker { 
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {} got a job; executing.", id);

            job();
        });
        Worker {
            id: id,
            thread: thread
        }
    }

    // pub fn run<C: 'static>(&mut self, closure: C)
    // where
    //     C: FnOnce() + Send + 'static
    // {
    //     self.thread = Some(thread::spawn(closure));
    // }
}
