// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::ops::{AddAssign, Deref, DerefMut};
struct JobStatus {
    jobs_completed: u32,
}

impl AddAssign for JobStatus
{
	fn add_assign(&mut self, other: Self)
	{
		*self = Self {
			jobs_completed: self.jobs_completed + other.jobs_completed
		};
	}
}

impl Deref for JobStatus
{
	type Target = u32;

	fn deref(&self) -> &Self::Target
	{
		&self.jobs_completed
	}
}

impl DerefMut for JobStatus
{
	fn deref_mut(&mut self) -> &mut u32
	{
		&mut self.jobs_completed
	}
}

fn main() {
    let status = Arc::new(Mutex::new( JobStatus { jobs_completed: 0 } ));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
			thread::sleep(Duration::from_millis(250));
			let mut num = status_shared.lock().unwrap();
            *num += JobStatus{ jobs_completed: 1 };
        }
	});
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
