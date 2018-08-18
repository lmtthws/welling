use std::env;
use std::fs;
use std::path::Path;
use std::io;

fn main() {
	// let args: Vec<String> = env::args().collect();
	// println!("{}", args);

	let base_out_path = Path::new("..\\target").join(env::var("PROFILE").unwrap());
	println!("{}", base_out_path.to_str().unwrap());
	
	for path in vec!["views", "scripts" ]
	{
		let out_path = &base_out_path.join(path);
		
		if out_path.exists() {
			fs::remove_dir_all(&out_path).unwrap();
		}

		let view_dir = Path::new("./").join(path);
		
		copy_dir_contents(&out_path, &view_dir).unwrap();
	}

}

fn copy_dir_contents(out_dir: &Path, src_dir: &Path) -> io::Result<()> {
	fs::create_dir(out_dir).unwrap();

	for entry in fs::read_dir(src_dir)? {
		let entry = entry?;
		let entry = entry.path();
		let name = entry.file_name().unwrap();
		if entry.is_dir() {
			let out_path = out_dir.join(Path::new(name));
			copy_dir_contents(&out_path, &entry)?;
		} else {
			let out_file = out_dir.join(name);
			fs::copy(&entry, out_file)?;
		}
	}

	Ok(())
}