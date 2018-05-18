use std::env;
use std::fs;
use std::path::Path;
use std::io;

fn main() {
	let out_path = Path::new("..\\target").join(env::var("PROFILE").unwrap());
	println!("{}", out_path.to_str().unwrap());
	let out_path = out_path.join("views");
	
	if out_path.exists() {
		fs::remove_dir_all(&out_path).unwrap();
	}

	let view_dir = Path::new("./views");
	copy_dir_contents(&out_path, view_dir).unwrap();
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