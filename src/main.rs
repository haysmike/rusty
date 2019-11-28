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
    match entry.metadata() {
        Ok(metadata) => match metadata.modified() {
            Ok(modified) => println!("{:?}", modified),
            Err(err) => eprint!(
                "Couldn't get modified time for {:?}: {:?}",
                entry.path(),
                err
            ),
        },
        Err(err) => eprint!("Couldn't get metadata for {:?}: {:?}", entry.path(), err),
    }
}

fn main() {
    let visit_result = visit_dirs(Path::new("."), &cb);
    match visit_result {
        Ok(()) => (),
        Err(err) => eprint!("{}", err),
    }
}
