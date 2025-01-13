extern crate log;

use std::fs;
use std::path::Path;
use git2::Repository;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum RepoError {
    DoesNotExist,
    IsNotADirectory,
    EmptyDirectory,
    NotReadableDirectory,
    NotGitRepo
}

pub fn check_directory(path: &str) -> Result<&Path, RepoError> {
    let path: &Path = Path::new(path);

    if !path.exists() {
        error!("La directory non esiste.");
        return Err(RepoError::DoesNotExist);
    }

    if !path.is_dir() {
        error!("Il path non è una directory.");
        return Err(RepoError::IsNotADirectory);
    }

    match fs::read_dir(path) {
        Ok(mut entries) => {
            if entries.next().is_some() {
                info!("La directory esiste, è leggibile e non è vuota.");
                return Ok(path);
            } else {
                error!("La directory esiste, è leggibile ma è vuota.");
                return Err(RepoError::EmptyDirectory);
            }
        }
        Err(e) => {
            error!("La directory non è leggibile: {}", e);
            return Err(RepoError::NotReadableDirectory);
        }
    }
}

pub fn resolve_repo_git(path: &Path) -> Result<Repository, RepoError> {
    info!("verify if {:?} is a git repository path", path);
    
    match Repository::open(path) {
        Ok(repo) => Ok(repo),
        Err(e) => {
            error!("failed to open: {}", e);
            return Err(RepoError::NotGitRepo);
        },
    }
}

#[suitest::suite(abc_suite)]
#[suitest::suite_cfg(sequential = false, verbose = true)]
mod tests {
    use std::path::PathBuf;
    use rand::{distributions::Alphanumeric, Rng};
    use fs::{create_dir, remove_dir, remove_file, File};

    use suitest::{after_all, before_all};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[derive(Debug, Clone)]
    struct TestData {
        file: String,
        folder: String
    }

    #[before_all]
    fn setup() -> TestData {        
        let a_file: PathBuf = PathBuf::from("/tmp/").join("simple-file");
        File::create(&a_file).unwrap();

        let a_folder_path: String = format!("/tmp/simple-dir-{}", rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(42)
            .map(char::from)
            .collect::<String>()
        );

        create_dir(&a_folder_path).unwrap();        

        let td: TestData = TestData { 
            file: a_file.into_os_string().into_string().unwrap(), 
            folder: a_folder_path
        };

        td
    }

    #[after_all]
    fn tear_down(td: TestData) {
        println!("#### tear_down on {:?} ####", td);

        remove_file(td.file.clone()).unwrap();
        remove_dir(td.folder.clone()).unwrap();
    }

    #[test]
    fn tmp_is_always_available() {
        assert!(check_directory("/tmp").is_ok());
    }

    #[test]
    fn folder_that_does_not_exist() {
        assert_eq!(check_directory("/does-not-exist").err().unwrap(), RepoError::DoesNotExist);
    }

    #[test]
    fn with_path_that_is_a_file(td: TestData) {   
        assert_eq!(check_directory(&td.file).err().unwrap(), RepoError::IsNotADirectory);
    }

    #[test]
    fn with_empty_folder(td: TestData) {
        assert_eq!(check_directory(&td.folder).err().unwrap(), RepoError::EmptyDirectory);
    }
    
}