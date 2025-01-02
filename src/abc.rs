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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn tmp_is_always_available() {
        assert!(check_directory("/tmp").is_ok());
    }

    #[test]
    fn folder_that_does_not_exist() {
        assert_eq!(check_directory("/does-not-exist").err().unwrap(), RepoError::DoesNotExist);
    }

}