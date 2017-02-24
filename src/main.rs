extern crate dotfile;
use dotfile::dotfile::DotFile;

#[cfg(target_family = "unix")]
use std::os::unix::fs;

#[cfg(target_family = "windows")]
panic!("Windows is not supported!");

fn main() {
    let mut args = std::env::args();
    let _ = args.next(); // skip command name

    while let Some(path) = args.next() {
        let dotfile = match DotFile::new(&path) {
            Ok(dotfile) => dotfile,
            Err(msg)    => panic!("{}", msg),
        };

        if dotfile.exists { // check existence
            match fs::symlink(&dotfile.absolute_path, &dotfile.dotfile_path) { // if so, symlink
                Ok(_)  => {},
                Err(_) => println!("There was an error while trying to symlink {} to {}",
                    dotfile.absolute_path.display(), dotfile.dotfile_path.display()),
            }
        } else {
            // else, error
            panic!("File doesn't exist!");
        }
    }
}
