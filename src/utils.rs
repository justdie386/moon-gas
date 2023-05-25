use std::fs;
use std::env;
use git2::Repository;
pub fn clone(repo: String, folder: String){
    let url = repo.clone();
    let repo = match Repository::clone(&url, folder){
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    
}
pub fn mkdir(name: String){
    fs::create_dir(name);
}