use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::net::TcpStream;
use super::worker::Worker;
use super::message::Message;
use crate::network::Router;


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>
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
            workers,
            sender
        }
    }

    pub fn execute<P: 'static>(&self, tcp: TcpStream, process: P)
    where
        P: FnOnce(Router) + Send + 'static
    {   
        let job = Box::new(|| {
            process(Router::new(tcp));
        });
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
