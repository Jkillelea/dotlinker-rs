use std::fs;
use std::fmt;
use std::env;
pub use std::path::{Path, PathBuf};

pub struct DotFile {
    pub exists:        bool,
    pub basename:      PathBuf,
    pub absolute_path: PathBuf,
    pub dotfile_path:  PathBuf,
    homedir:           PathBuf,
}

impl fmt::Debug for DotFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "DotFile {{ \
            \n\texists:        {}\
            \n\tbasename:      {}\
            \n\tabsolute_path: {}\
            \n\tdotfile_path:  {}\
            \n\thomedir:       {}\
            \n}}",
            self.exists,
            self.basename.display(),
            self.absolute_path.display(),
            self.dotfile_path.display(),
            self.homedir.display())
    }
}

impl DotFile {
    pub fn new(p: & String) -> Result<DotFile, &'static str> {
        let dotfile = init(PathBuf::from(p));
        dotfile
    }

    #[allow(dead_code)]
    pub fn is_dotted(&self) -> bool {
        let first_char = match self.basename.to_str() {
            Some(string) => string.chars().nth(0),
            None => None,
        }.unwrap();

        if first_char == '.' {
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn dot(&self) -> PathBuf { // Getting less hacky

        if !self.is_dotted() {
            let mut dotfile_path = self.homedir.to_path_buf();
            dotfile_path.push(
                {".".to_string() + &self.basename.to_str().unwrap()}
            );
            dotfile_path
        } else {
            let mut dotfile_path = self.homedir.to_path_buf();
            dotfile_path.push(&self.basename);
            dotfile_path
        }
    }

    #[allow(dead_code)]
    fn undot(&self) -> PathBuf {
        if self.is_dotted() {
            let mut path = self.basename.to_str().unwrap();
            path = &path[1..path.len()];
            PathBuf::from(path)
        } else {
            PathBuf::from(&self.basename.to_str().unwrap())
        }
    }

}


fn init(relpath: PathBuf) -> Result<DotFile, &'static str> { // init handles preparing a DotFile struct
    let homedir = match env::home_dir() { // Option<PathBuf>
        Some(path) => path,
        _ => return Err("Can't find home direcotry!")
    };

    let absolute_path = PathBuf::from( &fs::canonicalize(relpath).expect("Given bogus file path!") );
    let basename      = PathBuf::from(absolute_path.file_name().expect("No file name!"));
    let dotfile_path  = PathBuf::new(); // temporary
    let exists        = absolute_path.exists();

    let mut dotfile = DotFile {
        exists:        exists,
        basename:      basename,
        absolute_path: absolute_path,
        dotfile_path:  dotfile_path,
        homedir:       homedir,
    };

    dotfile.dotfile_path = dotfile.dot();
    let dotfile = dotfile; // make immutable
    Ok(dotfile) // return
}
