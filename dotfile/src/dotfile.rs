use std::fs;
use std::fmt;
use std::env;
pub use std::path::PathBuf;

macro_rules! find {
    ($x:expr) =>
    (match $x { // Option<T>
        Some(thing) => thing,
        _ => return Err("Macro 'find!' couldn't find anything!")
    });
}

pub struct DotFile {
    pub exists:        bool,
    pub basename:      PathBuf, // PathBuf because we want ownership and editability
    pub absolute_path: PathBuf,
    pub dotfile_path:  PathBuf,
    homedir:           PathBuf,
}

fn init(relpath: &PathBuf) -> Result<DotFile, &'static str> { // init handles preparing a DotFile struct

    let homedir       = find!(env::home_dir());
    let absolute_path = PathBuf::from( &fs::canonicalize(relpath).expect("Given bogus file path!") );
    let basename      = PathBuf::from( absolute_path.file_name().expect("No file name!") );
    let exists        = absolute_path.exists();
    let dotfile_path  = PathBuf::new(); // temporary

    let mut dotfile = DotFile {
        exists:        exists,
        basename:      basename,
        absolute_path: absolute_path,
        dotfile_path:  dotfile_path, // temporary
        homedir:       homedir,
    };

    dotfile.dotfile_path = dotfile.dot();
    let dotfile = dotfile; // make immutable
    Ok(dotfile) // return
}

impl DotFile {
    #[allow(dead_code)]
    pub fn new(p: & String) -> Result<DotFile, &'static str> {
        let dotfile = init(&PathBuf::from(p)); // Result<DotFile, &'static str>
        dotfile
    }

    #[allow(dead_code)]
    pub fn is_dotted(&self) -> bool {
        let first_char = match self.basename.to_str() {
            Some(string) => string.chars().nth(0),
            None => return false,
        }.unwrap();

        if first_char == '.' {
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    fn dot(&self) -> PathBuf { // Getting less hacky

        let mut dotfile_path = self.homedir.to_path_buf();
        if !self.is_dotted() {
            dotfile_path.push(".".to_string() + &self.basename.to_str().unwrap());
        } else {
            dotfile_path.push(&self.basename);
        }
        dotfile_path
    }

    #[allow(dead_code)]
    fn undot(&self) -> PathBuf {
        let mut path = self.basename.to_str().unwrap();
        if self.is_dotted() {
            path = &path[1..]; // chop off first char
        }
        PathBuf::from(path)
    }

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
            self.homedir.display()
        )
    }
}
