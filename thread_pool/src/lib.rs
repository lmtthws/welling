use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
	num: usize,
	threads: Vec<Worker>,
	sender: mpsc::Sender<Job>
}

pub struct PoolCreationError<'a> {
	pub message: &'a str
}

type JobReceiver<T> = Arc<Mutex<mpsc::Receiver<T>>>;

struct Worker {
	id: usize,
	handle: thread::JoinHandle<()>,
}

impl Worker {
	fn new(id: usize, receiver: JobReceiver<Job>) -> Worker {
		Worker {
			id,
			handle: thread::spawn(|| {
				receiver;
			}),
		}
	}
}



struct Job {
}


impl ThreadPool {

	/// Create a new ThreadPool.
    ///
    /// The COUNT is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
	pub fn new<'a>(count: usize) -> Result<ThreadPool, PoolCreationError<'a>> {
		if count < 1 {
			return Err(PoolCreationError{ message: "Thread count must be grater than 0"});
		}

		let mut threads = Vec::with_capacity(count);

		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));


		for id in 0..count {
			threads.push(Worker::new(id, Arc::clone(&receiver)));
		}

		Ok(ThreadPool{
			num:  count,
			threads,
			sender
		})
	}

	pub fn execute<F>(&self, _f: F) 
		where F: FnOnce() + Send + 'static {

	}
}