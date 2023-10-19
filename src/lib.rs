use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::{sync::mpsc, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    let start_time = std::time::Instant::now();
                    println!("Worker {id} got a job; executing.");

                    job();
                    println!("Worker {} finished job in {:?}", id, start_time.elapsed());
                }
                Err(_) => {
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool {
    pub fn new(num: usize) -> ThreadPool {
        let mut workers = Vec::with_capacity(num);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        for id in 0..num {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }
        ThreadPool{workers, sender: Some(sender)}
    }

    pub fn execute<F>(&self, f: F)
    where
    F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // no more jobs will be sent
        drop(self.sender.take());
        println!("Drop ThreadPool: closed channel.");

        for worker in &mut self.workers {

            if let Some(thread) = worker.thread.take() {
                // wait for thread to finish
                thread.join().unwrap();
            }
            println!("Drop ThreadPool: worker {} thread joined/finished", worker.id);

        }
    }
}

