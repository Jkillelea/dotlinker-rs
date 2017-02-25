extern crate dotfile;
use dotfile::dotfile::DotFile;
use std::os::unix::fs;

fn main() {
    let mut args = std::env::args();
    let _ = args.next(); // skip command name

    while let Some(path) = args.next() {
        let dotfile = match DotFile::new(&path) {
            Ok(dotfile) => dotfile,
            Err(msg)    => panic!("{}", msg),
        };

        // println!("{:?}", dotfile);
        if dotfile.exists { // check existence
            match fs::symlink(&dotfile.absolute_path, &dotfile.dotfile_path) { // if so, symlink
                Ok(_)  => {},
                Err(_) => {
                    println!("There was an error while trying to symlink {} to {}",
                        &dotfile.absolute_path.display(), &dotfile.dotfile_path.display());
                        println!("{:?}", &dotfile);
                    },
            }
        } else { // error
            println!("{:?}", dotfile);
            panic!("File doesn't exist!");
        }
    }
}
