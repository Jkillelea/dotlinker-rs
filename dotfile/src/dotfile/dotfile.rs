use std::path::PathBuf;
use std::fs;
use std::env;
use super::util;

pub struct DotFile {
    pub exists:        bool,
    pub basename:      PathBuf, // PathBuf because we want ownership and editability
    pub absolute_path: PathBuf,
    pub dotfile_path:  PathBuf,
    pub homedir:       PathBuf,
}

impl DotFile {
    #[allow(dead_code)]
    pub fn new(p: &String) -> Result<DotFile, &'static str> {
        let dotfile = DotFile::init(&PathBuf::from(p)); // Result<DotFile, &'static str>
        dotfile
    }

    #[allow(dead_code)]
    pub fn is_dotted(&self) -> bool {
        util::is_dotted(&self.basename)
    }

    #[allow(dead_code)]
    fn dot(&self) -> PathBuf { // Getting less hacky
        util::dot(&self.basename, &self.homedir)
    }

    #[allow(dead_code)]
    fn undot(&self) -> PathBuf {
        let mut path = self.basename.to_str().unwrap();
        if self.is_dotted() {
            path = &path[1..]; // chop off first char
        }
        PathBuf::from(path)
    }

    fn init(relpath: &PathBuf) -> Result<DotFile, &'static str> { // init handles preparing a DotFile struct
        let absolute_path = fs::canonicalize(relpath).unwrap();
        let basename = PathBuf::from(absolute_path.file_name().unwrap());
        let exists   = absolute_path.exists();
        let homedir  = match env::home_dir() {
            Some(path) => path as PathBuf,
            None       => return Err("No home directory was found!")
        };
        let dotfile_path = util::dot(&basename, &homedir);

        let dotfile = super::DotFile {
            exists:        exists,
            basename:      basename,
            absolute_path: absolute_path,
            dotfile_path:  dotfile_path,
            homedir:       homedir,
        };

        Ok(dotfile) // return
    }

}