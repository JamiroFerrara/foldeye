extern crate job_scheduler;
use job_scheduler::*;
use std::time::Duration;
use crate::directory::*;

pub struct Chron {
    pub path: String,
    pub interval: String,
    pub directory: Directory,
}

impl Chron {
    pub fn new(path: String, interval: String, directory: Directory) -> Self {
        Self {
            path,
            interval,
            directory
        }
    }

    pub fn watch_folder(mut self, path: &str) -> Result<(), std::io::Error>{
        let mut scheduler = JobScheduler::new();

        scheduler.add(Job::new(self.interval.parse().unwrap(), || {
            let dir = Directory::new(path); //Exp
            match dir {
                Ok(d) => {
                    let (_inserted, _deleted) = &self.directory.compare(&d);
                    self.directory = d;
                }
                Err(_) => { }
            }
        }));

        loop {
            scheduler.tick();
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

