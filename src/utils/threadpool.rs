use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::net::TcpStream;
use crate::utils::Job;
use crate::utils::Worker;
use crate::network::Router;


pub struct ThreadPool {
    index: usize,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

impl ThreadPool {
    pub fn new(thread_count: usize) -> ThreadPool {
        assert!(thread_count > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(thread_count);

        for id in 0..thread_count {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            index: 0,
            workers,
            sender
        }
    }

    pub fn execute<P: 'static>(&self, tcp: TcpStream, process: P)
    where
        P: FnOnce(Router) + Send + 'static
    {   
        let router = Router::new(tcp);
        let job = Box::new(move || {
            process(router);
        });
        self.sender.send(job).unwrap();
    }
}