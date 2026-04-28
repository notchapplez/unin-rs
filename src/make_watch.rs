use std::{
	collections::HashMap,
	fs,
	io::Read,
	os::unix::fs::MetadataExt,
	path::{Path, PathBuf},
};
use std::process::Stdio;
use walkdir::WalkDir;

fn scan_meta(root: &Path) -> HashMap<PathBuf, (u64,u64,u64)> {
	let mut m = HashMap::new();
	for e in WalkDir::new(root).into_iter().filter_map(Result::ok) {
		if e.file_type().is_file() {
			if let Ok(md) = fs::metadata(e.path()) {
				m.insert(e.path().to_path_buf(), (md.dev(), md.ino(), md.size()));
			}
		}
	}
	m
}
fn is_elf(path: &Path) -> bool {
	if let Ok(mut f) = fs::File::open(path) {
		let mut b = [0u8;4];
		if f.read_exact(&mut b).is_ok() {
			return b == [0x7f,b'E', b'L', b'F'];
		}
	}
	false
}
pub fn find_bin(dir: PathBuf) -> Vec<PathBuf> {
	let before = scan_meta(&dir);

	let num = num_cpus::get();
	let mut child = std::process::Command::new("make")
		.args(&["-j", &num.to_string()])
		.current_dir(&dir)
		.stderr(Stdio::piped())
		.stdout(Stdio::null())
		.spawn()
		.expect("failed to execute process");

	if let Some(mut s) = child.stderr.take() {
		let mut _buf = String::new();
		let _ = s.read_to_string(&mut _buf);
	}
	let _ = child.wait();

	let mut results = Vec::new();
	for e in WalkDir::new(&dir).into_iter().filter_map(Result::ok) {
		if !e.file_type().is_file() {
			continue;
		}
		let p = e.path().to_path_buf();
		if let Ok(md) = fs::metadata(&p) {
			let is_new_or_changed = match before.get(&p) {
				Some((dev, ino, size)) => md.dev() != *dev || md.ino() != *ino || md.size() != *size,
				None => true,
			};
			if !is_new_or_changed {
				continue;
			}
			let exec = md.mode() & 0o111 != 0;
			if exec || is_elf(&p){
				results.push(p)
			}
		}
	}
	results.sort();
	results

}
