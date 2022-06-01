use std::thread;
use crate::error::Result;
use std::sync::{mpsc, Arc, Mutex};
use std::panic;
use std::cell::RefCell;

type Job = Box<dyn FnOnce() -> Result<()> + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub trait ThreadPool{

    fn new(threads: u32) -> Result<Self>
    where Self: Sized;

    fn spawn<F>(&self, job:F)
    where F: FnOnce() -> Result<()> + Send + 'static;

}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn( move || loop {
            eprintln!("Inside worker new");
            let message = receiver.lock().expect("Failed to acquire mutex lock on receiver channel").recv().expect("Failed to receive message from channel"); //TODO! Better error handling
            eprintln!("Received new job");

            match message {
                Message::NewJob(job) => {
                    eprintln!("Doing new job");
                    job();
                    
                },
                Message::Terminate => {
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

pub struct NaiveThreadPool;

impl ThreadPool for NaiveThreadPool {
    fn new(threads: u32) -> Result<NaiveThreadPool> {
        Ok(NaiveThreadPool)
    }
    
    fn spawn<F>(&self, job:F)
    where 
    F: FnOnce() -> Result<()> + Send + 'static
    {
        thread::spawn(|| { job()});
    }
}

pub struct SharedQueueThreadPool{
    workers: RefCell<Vec<Worker>>,
    sender: mpsc::Sender<Message>,
    receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
    threads: u32,
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> Result<SharedQueueThreadPool> {
        assert!( threads > 0 ); //TODO! Better error handling. Proporate error here?

        let (sender, receiver) = mpsc::channel::<Message>();

        let receiver = Arc::new(Mutex::new(receiver));
        let vec_workers: RefCell<Vec<Worker>> = RefCell::new(vec![]);

        for i in 0..threads {
            let receiver = receiver.clone();
            vec_workers.borrow_mut().push(Worker::new(i as usize, receiver));
        }

        Ok(SharedQueueThreadPool{ workers: vec_workers, sender, threads, receiver})
    }
    
    fn spawn<F>(&self, job: F) 
    where F: FnOnce() -> Result<()> + Send + 'static
    {

        let workers_num = self.workers.borrow().len().clone();

        let thread_count = self.threads.clone() as usize;

        // if workers_num < thread_count {
        //     for i in 0..(thread_count - workers_num) {
        //         let receiver = self.receiver.clone();
        //         let id = i + workers_num;
        //         self.workers.borrow_mut().push(Worker::new(id, receiver));    
        //     }

        // }

        // for worker in self.workers.borrow_mut().iter_mut() {
        //     eprintln!("worker thread: {:?}", worker.thread);
        //     if let Some(thread) = worker.thread.take() {
        //         thread.join();
        //     }
        // }

        eprintln!("workers_num: {}", workers_num);
        eprintln!("workers_num: {}", thread_count);

        let job = Box::new(job);
        self.sender.send(Message::NewJob(job));

        eprintln!("Job sent");

    }
}

impl Drop for SharedQueueThreadPool {
    fn drop(&mut self) {
        println!("Dropping ThreadPool and terminating workers.");

        for _ in self.workers.borrow().iter() {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in self.workers.borrow_mut().iter_mut() {
            println!("Shutting down worker. ID: {}", worker.id);

            // worker.thread.take().unwrap().join().unwrap();

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

//TODO! Placeholder for Rayon crate
pub struct RayonThreadPool;

impl ThreadPool for RayonThreadPool {
    fn new(threads: u32) -> Result<RayonThreadPool> {
        todo!()
    }
    
    fn spawn<F>(&self, job: F) 
    where F: FnOnce() -> Result<()> + Send + 'static
    {
        todo!()
    }
}
