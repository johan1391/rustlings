// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)


use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Copy, Clone)]
struct JobStatus {
    jobs_completed: u32,
}

impl JobStatus {
    fn increment(&mut self) -> Self{
        self.jobs_completed += 1;
        let new_number = self.jobs_completed + 1;
        JobStatus{jobs_completed: new_number}
    }
    
    fn value(self) -> u32 {
        self.jobs_completed
    }
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let shared_status = status.clone();
    thread::spawn(move || {
        let count = Arc::clone(&status);
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut num = count.lock().unwrap();

            num.increment();
            
        }
        // status_shared.jobs_completed = count;
    });
    while shared_status.lock().unwrap().value() < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
