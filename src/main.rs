use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

// https://doc.rust-lang.org/std/fs/fn.read_dir.html#examples
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn cb(entry: &DirEntry) -> () {
    let metadata_result = entry.metadata();
    if metadata_result.is_ok() {
        // println!("{:?}", metadata);
        let metadata = metadata_result.unwrap();
        let modified_result = metadata.modified();
        if modified_result.is_ok() {
            println!("{:?}", modified_result.unwrap());
        } else {
            eprint!(
                "Couldn't get modified time for {:?}: {:?}",
                entry.path(),
                modified_result.unwrap_err()
            );
        }
    } else {
        eprint!(
            "Couldn't get metadata for {:?}: {:?}",
            entry.path(),
            metadata_result.unwrap_err()
        );
    }
}

fn main() {
    if let Err(err) = visit_dirs(Path::new("."), &cb) {
        eprint!("{}", err);
    }
}
