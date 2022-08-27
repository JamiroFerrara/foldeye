extern crate job_scheduler;
use job_scheduler::*;
use std::time::Duration;

use crate::directory::*;

pub fn start(path: &str) {
    let mut scheduler = JobScheduler::new();

    // Adding a task to scheduler to execute it in every second
    scheduler.add(Job::new("0 5 * * * *".parse().unwrap(), || {
        let dir = Directory::new(path);
        match dir {
            Ok(d) => {
                let files = d.all_dirs();
                println!("{:?}", files);
            }
            Err(_) => { }
        }
    }));

    loop {
        scheduler.tick();
        std::thread::sleep(Duration::from_millis(100));
    }
}
