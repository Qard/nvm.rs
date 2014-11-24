use std::io::fs::{PathExtensions};
use std::io::{fs,IoError,IoResult};
use std::os;

#[deriving(Show)]
pub enum NVMError {
  None,
  NotImplemented(&'static str),
  InstallError(&'static str),
  GenericError(&'static str),
  UseError(&'static str)
}

pub fn home () -> Path {
  os::homedir().unwrap().join(Path::new(".nvm"))
}

fn path_to_version (path: &Path) -> Result<String, NVMError> {
  let pathString = match path.filename_str() {
    None => return Err(NVMError::GenericError("Failed to read symlink")),
    Some(pathString) => pathString
  };

  let p = Path::new(pathString.slice_from(1));

  let version = match p.as_str() {
    None => return Err(NVMError::GenericError("Failed to determine version for path")),
    Some(version) => version
  };

  Ok(version.into_string())
}

pub fn current_version () -> Result<String, NVMError> {
  let path = match fs::readlink(&home().join(Path::new("current"))) {
    Err(e) => return Err(NVMError::GenericError("Failed to read symlink")),
    Ok(path) => path
  };

  installed_versions();

  path_to_version(&path)
}

fn installed_versions () -> IoResult<Vec<String>> {
  let contents = try!(fs::readdir(&home()));

  let strings = contents.iter()
    .filter(|path| path.is_dir())
    .inspect(|path| println!("path is: {}", path.as_str()))
    .map(|path| path_to_version(path).unwrap())
    .collect::<Vec<String>>();

  println!("{}", strings)

  Ok(vec!["".into_string()])
}

pub fn install (version: String) -> Result<String, NVMError> {
  Err(NVMError::NotImplemented("Install is not yet implemented"))
}

pub fn use_version (version: String) -> Result<String, NVMError> {
  Err(NVMError::NotImplemented("Use is not yet implemented"))
}
