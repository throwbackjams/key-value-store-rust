use std::thread;
use crate::error::Result;

pub trait ThreadPool{

    fn new(threads: u32) -> Result<Self>
    where Self: Sized;

    fn spawn<F>(&self, job:F)
    where F: FnOnce() + Send + 'static;

}

pub struct NaiveThreadPool;

impl ThreadPool for NaiveThreadPool {
    fn new(threads: u32) -> Result<NaiveThreadPool> {
        Ok(NaiveThreadPool)
    }
    
    fn spawn<F>(&self, job:F)
    where F: FnOnce() + Send + 'static
    {
        thread::spawn(|| { job();});
    }
}

pub struct SharedQueueThreadPool;

impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> Result<SharedQueueThreadPool> {
        todo!()
    }
    
    fn spawn<F>(&self, job: F) 
    where F: FnOnce() + Send + 'static
    {
        todo!()
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
