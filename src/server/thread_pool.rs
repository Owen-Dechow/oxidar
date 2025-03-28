use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use super::Oxidar;

type Job = Box<dyn FnOnce() + Send>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub(crate) fn new(
        oxidar: Arc<Oxidar>,
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    ) -> Self {
        // todo!()
        // THIS MIGHT FAIL TO BUILD A THREAD
        let noxidar = oxidar.clone();
        let worker = Self {
            id,
            thread: Some(thread::spawn(move || loop {
                match receiver.lock().unwrap().recv() {
                    Ok(job) => {
                        noxidar.log(format!("Worker {id} given job."));
                        job();
                    }
                    Err(err) => {
                        noxidar.log(format!("Completed shutdown of worker {id} exiting: {err}."));
                        break;
                    }
                }
            })),
        };

        oxidar.log(format!("Worker {id} created."));
        return worker;
    }
}

pub(crate) struct ThreadPool {
    workers: Vec<Worker>,
    jobs: Option<mpsc::Sender<Job>>,
    oxidar: Arc<Oxidar>,
}

impl ThreadPool {
    pub(crate) fn new(oxidar: Arc<Oxidar>, size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(oxidar.clone(), id, receiver.clone()));
        }

        Self {
            workers,
            jobs: Some(sender),
            oxidar,
        }
    }

    pub(crate) fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.jobs.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.jobs.take());

        for worker in &mut self.workers.drain(..) {
            self.oxidar
                .log(format!("Starting shut down of worker {}", worker.id));
            if let Some(thread) = worker.thread {
                thread.join().unwrap()
            };
        }
    }
}
