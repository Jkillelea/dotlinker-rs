use std::fs;
use std::fmt;
use std::env;
pub use std::path::PathBuf;

macro_rules! is_dotted {
    ($x: expr) => (($x.to_str().unwrap().chars().nth(0) == Some('.')));
}

pub struct DotFile {
    pub exists:        bool,
    pub basename:      PathBuf, // PathBuf because we want ownership and editability
    pub absolute_path: PathBuf,
    pub dotfile_path:  PathBuf,
    pub homedir:       PathBuf,
}

fn init(relpath: &PathBuf) -> Result<DotFile, &'static str> { // init handles preparing a DotFile struct
    let absolute_path = fs::canonicalize(relpath).unwrap();
    let basename = PathBuf::from(absolute_path.file_name().unwrap());
    let exists   = absolute_path.exists();
    let homedir  = match env::home_dir() {
        Some(path) => path as PathBuf,
        None       => return Err("No home directory was found!")
    };
    let dotfile_path = dot(&basename, &homedir);

    let dotfile = DotFile {
        exists:        exists,
        basename:      basename,
        absolute_path: absolute_path,
        dotfile_path:  dotfile_path,
        homedir:       homedir,
    };

    Ok(dotfile) // return
}

impl DotFile {
    pub fn new(p: &String) -> Result<DotFile, &'static str> {
        let dotfile = init(&PathBuf::from(p)); // Result<DotFile, &'static str>
        dotfile
    }

    #[allow(dead_code)]
    pub fn is_dotted(&self) -> bool {
        is_dotted!(self.basename)
    }

    #[allow(dead_code)]
    fn dot(&self) -> PathBuf { // Getting less hacky
        dot(&self.basename, &self.homedir)
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

fn dot(basename: &PathBuf, homedir: &PathBuf) -> PathBuf {
    let mut dotfile_path = homedir.to_owned(); // to owned

    if !(is_dotted!(basename)) { // if not already dotted
        dotfile_path.push(".".to_string() + basename.to_str().unwrap()); // dot it
    } else { // else do nothing
        dotfile_path.push(basename);
    }
    dotfile_path
}
