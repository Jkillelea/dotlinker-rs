use std::env;
use super::*;

#[test]
fn finds_home_dir() {
    match env::home_dir() {
        Some(_) => {},
        None => {assert!(false, "Couldn't find home dir!");}
    };
}

#[test]
fn generates_a_dotfile() {
    let _ = match DotFile::new("/home/jacob/.bashrc") {
        Ok(dotfile) => dotfile,
        Err(e) => panic!(e)
    };
}

#[test]
fn checks_existence() {
    let dotfile = match DotFile::new("/home/jacob/.bashrc") {
        Ok(dotfile) => dotfile,
        Err(e) => panic!(e)
    };
    if !dotfile.exists {
        panic!("created a dotfile which doesn't exist!");
    }

}
