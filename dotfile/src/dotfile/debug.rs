use std::fmt;
use super::DotFile;

impl fmt::Debug for DotFile {
    // can also use the format string :#? for automatic pretty output
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
