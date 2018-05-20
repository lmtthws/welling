use std::thread;


pub struct ThreadPool {
	_num: usize,
	_threads: Vec<thread::JoinHandle<()>>,
}

pub struct PoolCreationError<'a> {
	pub message: &'a str
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

		let threads = Vec::with_capacity(count);

		for _ in 0..count {

		}

		Ok(ThreadPool{
			_num:  count,
			_threads: threads
		})
	}

	pub fn execute<F>(&self, _f: F) 
		where F: FnOnce() + Send + 'static {

	}
}