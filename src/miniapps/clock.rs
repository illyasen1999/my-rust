use chrono;
use std::{time, thread};

pub fn clock_fn() {
    loop {
        let local_time_now = chrono::Local::now().format("%H:%M:%S");
        let run_seconds = time::Duration::from_secs(1);
        println!("Time: {}", local_time_now);
        thread::sleep(run_seconds);
    }
}

// IDEA: make a program where you create a file and edit it and if there are changes in the file add a Log file to track the date and time of the changes using concepts from clock.rs and filecreate.rs