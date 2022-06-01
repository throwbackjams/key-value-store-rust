use std::thread;
use crate::error::Result;
use crossbeam::channel::{self, Receiver, Sender};

pub trait ThreadPool{

    fn new(threads: u32) -> Result<Self>
    where Self: Sized;

    fn spawn<F>(&self, job:F)
    where F: FnOnce() + Send + 'static;

}

#[derive(Debug, Clone)]
struct Worker(Receiver<Box<dyn FnOnce() + Send + 'static>>);

impl Drop for Worker {
    fn drop(&mut self) {
        println!("Dropping Worker");

        if thread::panicking() {
            let worker_clone = self.clone();
            thread::spawn(move || do_job(worker_clone));
        }

    }
}

fn do_job(worker: Worker) {
    loop {
        match worker.0.recv() {
            Ok(task) => {
                task();
            }
            Err(_) => eprintln!("Error doing task")
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
    F: FnOnce() + Send + 'static
    {
        thread::spawn(|| { job()});
    }
}

pub struct SharedQueueThreadPool{
    sender: Sender<Box<dyn FnOnce() + Send + 'static>>
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> Result<SharedQueueThreadPool> {
        assert!( threads > 0 ); //TODO! Better error handling

        let (sender, receiver) = channel::unbounded::<Box<dyn FnOnce() + Send + 'static>>();

        for i in 0..threads {
            let worker = Worker(receiver.clone());
            thread::spawn(move|| do_job(worker));
        }

        Ok(SharedQueueThreadPool{ sender })
    }
    
    fn spawn<F>(&self, job: F) 
    where F: FnOnce() + Send + 'static
    {

        let job = Box::new(job);
        let _result = self.sender.send(job);
    }
}

//TODO! Placeholder for Rayon crate
pub struct RayonThreadPool;

impl ThreadPool for RayonThreadPool {
    fn new(threads: u32) -> Result<RayonThreadPool> {
        todo!()
    }
    
    fn spawn<F>(&self, job: F) 
    where F: FnOnce() + Send + 'static
    {
        todo!()
    }
}
