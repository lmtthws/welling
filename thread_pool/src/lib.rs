use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ThreadPool {
	_num: usize,
	_threads: Vec<Worker>,
	sender: mpsc::Sender<Job>
}

pub struct PoolCreationError<'a> {
	pub message: &'a str
}

type JobReceiver<T> = Arc<Mutex<mpsc::Receiver<T>>>;

struct Worker {
	_id: usize,
	_handle: thread::JoinHandle<()>,
}

trait FnBox {
	fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
	fn call_box(self: Box<F>) {
		(*self)()
	}
}

type Job = Box<FnBox + Send + 'static>;


impl Worker {
	fn new(id: usize, receiver: JobReceiver<Job>) -> Worker {
		Worker {
			_id: id,
			_handle: thread::spawn(move || {
				loop {
					let job = receiver.lock().unwrap().recv().unwrap();

					println!("Worker {} got a job; executing.", id);

					job.call_box();
				}
			}),
		}
	}
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
			_num:  count,
			_threads: threads,
			sender
		})
	}

	pub fn execute<F>(&self, f: F) 
		where F: FnOnce() + Send + 'static {
			let job = Box::new(f);
			self.sender.send(job).unwrap();
	}
}