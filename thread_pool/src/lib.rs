
pub struct ThreadPool {
	_num: usize,
}

impl ThreadPool {
	pub fn new(count: usize) -> ThreadPool {
		ThreadPool{
			_num:  count
		}
	}

	pub fn execute<F>(&self, _f: F) 
		where F: FnOnce() + Send + 'static {

	}
}