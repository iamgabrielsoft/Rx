
use crate::error::*; 

use std::fs::File;
use std::path::PathBuf; 
use serde_derive::{Serialize, Deserialize};
use serde_json; 
use std::path::Path; 


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Operation {
    pub source: PathBuf,
    pub  target: PathBuf
}



#[derive(Serialize, Deserialize)]
struct DumpFormat {
    date: String, 
    operations: Operations
}


pub type Operations = Vec<Operation>; 


//dump to operations
pub fn dump_to_file(operations: &[Operation]) -> Result<()> {
    let now = chrono::Local::now(); 
    
    
    let dump = DumpFormat{
        date: now.format("%Y-%m-%d %H:%M:%S").to_string() , 
        operations: operations.to_vec(),
    }; 

    
    //a json dump file with info on your last update
    let filename = "rx-".to_string() + &now.format("%Y-%m-%d_%H%M%S").to_string() + ".json";


    let file = match File::create(&filename) {
        Ok(file) => file, 
        Err(_) => {
            return Err(Error {
                kind: ErrorKind::CreateFile, 
                value: Some(filename),
            })
        }
    };


    match serde_json::to_writer_pretty(file, &dump){
        Ok(_) => Ok(()),
        Err(_) => Err(Error {
            kind: ErrorKind::JsonParse,
            value: Some(filename),
        }),
    }

}



pub fn read_from_file(filepath: &Path) -> Result<Operations> {
    let file = match File::open(&filepath) {
        Ok(file) => file,
        Err(_) => {
            return Err(Error {
                kind: ErrorKind::ReadFile,
                value: Some(filepath.to_string_lossy().to_string()),
            })
        }
    };


    let dump: DumpFormat = match serde_json::from_reader(file) {
        Ok(dump) => dump,
        Err(_) => {
            return Err(Error {
                kind: ErrorKind::JsonParse,
                value: Some(filepath.to_string_lossy().to_string()),
            })
        }
    };


    Ok(dump.operations)
}



#[cfg(test)]
mod test {
    use tempfile::tempfile;

    use crate::config::RunMode;
    use crate::fileutils::{PathList, create_symlink, get_unique_filename};

    use super::*;
    use std::fs; 
    use std::io::prelude::*;  
    extern crate tempfile; 
    


    #[test]
    fn unique_name() {
        let tempdir = tempfile::tempdir().expect("error when creating directory");
        println!("Runing test in '{:?}'", tempdir); 

        let temp_path = tempdir.path().to_str().unwrap(); 
        let mock_files: PathList = vec![
            [temp_path, "test_file_1"].iter().collect(), 
            [temp_path, "test_file_2"].iter().collect(),
            [temp_path, "test_file_3"].iter().collect(),
        ]; 

        
        //iterate through our mock_iles
        for file in &mock_files {
            fs::File::create(file).expect("Error creating mock files..."); 
        }


        let symlink = PathBuf::from(format!("{}/test_file_2/", temp_path)); 
        create_symlink(&mock_files[0], &symlink).expect("Error creating symlink."); 


        //let check for broken symlink 
        let broken_symlink = PathBuf::from(format!("{}/test_file_2", temp_path)); 
        create_symlink(&PathBuf::from("broken_link"), &broken_symlink)
            .expect("Error creating broken symlink");

            
        //then create a new file

        let new_file:PathBuf = [temp_path, "test_file_2"].iter().collect(); 
        assert_eq!(get_unique_filename(&mock_files[0], ""), new_file); 
    }


    #[test]
    #[warn(unused_variables)]
    fn get_file_list() {
        let mock_files:Vec<String> = [
            "test_file_1.txt".to_string(),
            "test_file_2.txt".to_string(),
            "test_file_3.txt".to_string(),
        ].to_vec(); 

        
        let mode = RunMode::Simple(mock_files);



        println!("file list, {:?}", mode)
    }
}