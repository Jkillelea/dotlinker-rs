use std::fs;
use std::env;
use std::path::PathBuf;
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
    pub fn new(p: &str) -> Result<DotFile, String> {
        let dotfile = DotFile::init(&PathBuf::from(p));
        dotfile
    }

    #[allow(dead_code)]
    pub fn is_dotted(&self) -> bool {
        util::is_dotted(&self.basename)
    }

    #[allow(dead_code)]
    pub fn dot(&self) -> Option<PathBuf> { // Getting less hacky
        util::dot(&self.basename, &self.homedir)
    }

    #[allow(dead_code)]
    pub fn undot(&self) -> PathBuf {
        let mut path = self.basename.to_str().unwrap();
        if self.is_dotted() {
            path = &path[1..]; // chop off first char
        }
        PathBuf::from(path)
    }

    fn init(relpath: &PathBuf) -> Result<DotFile, String> { // init handles preparing a DotFile struct
        let absolute_path = match fs::canonicalize(relpath) {
            Ok(abspath) => abspath,
            Err(e)      => return Err(e.to_string()) // canonicalize would return an io::Error normally
        };
        let homedir = match env::home_dir() {
            Some(path) => path as PathBuf,
            None       => return Err(String::from("No home directory was found!"))
        };
        let basename = match absolute_path.file_name() {
            Some(fname) => PathBuf::from(fname),
            None        => return Err(String::from("Couldn't get file name!"))
        };
        let dotfile_path = match util::dot(&basename, &homedir) {
            Some(path) => path,
            None       => return Err(String::from("Error trying to dot the basename!"))
        };
        let exists = absolute_path.exists();

        let dotfile = DotFile {
            exists:        exists,
            basename:      basename,
            absolute_path: absolute_path,
            dotfile_path:  dotfile_path,
            homedir:       homedir,
        };

        Ok(dotfile) // return
    }

}
