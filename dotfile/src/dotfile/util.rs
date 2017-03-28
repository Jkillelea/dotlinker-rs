use std::path::PathBuf;

// macro_rules! is_dotted {
//     ($x: expr) => (($x.to_str().unwrap().chars().nth(0) == Some('.')));
// }

pub fn is_dotted(p: &PathBuf) -> bool {
    let s = match p.to_str() {
        Some(s) => s,
        None    => return false
    };

    s.chars().nth(0) == Some('.')
}

pub fn dot(basename: &PathBuf, homedir: &PathBuf) -> Option<PathBuf> {
    let mut dotfile_path = homedir.to_owned(); // to owned

    if !(is_dotted(basename)) { // if not already dotted
        let base_str = match basename.to_str() {
            Some(s) => s,
            None => return None
        };
        dotfile_path.push(".".to_string() + base_str); // dot it
    } else { // else do nothing
        dotfile_path.push(basename);
    }

    Some(dotfile_path)
}
