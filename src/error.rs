use std::result; 

pub type Result<T> = result::Result<T, Error>; 

//error type here
#[derive(Debug)]
pub enum  ErrorKind {
    CreateBackup, 
    CreateFile, 
    CreateSymlink, 
    ExistingPath, 
    JsonParse, 
    ReadFile, 
    Rename, 
    SameFilename, 
    SolveOrder
}


#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind, 
    pub value: Option<String>
}


impl Error  {
    pub fn description(&self) -> &str {
        use self::ErrorKind::*; 
        match self.kind {
            CreateBackup => "Cannot create a backup of", 
            CreateFile => "Cannot create file", 
            CreateSymlink => "Cannot create symlink", 
            ExistingPath => "Conflict with existing path", 
            JsonParse => "Cannot parse JSON  file",
            ReadFile => "Cannot open/read file",
            Rename => "Cannot Rename", 
            SameFilename => "Files will have the same name", 
            SolveOrder => "Cannot solve sorting problem"
        }
    }
}