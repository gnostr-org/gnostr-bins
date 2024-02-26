use git2::{Repository, IndexAddOption};
use std::path::Path;
use std::process;
use std::ffi::OsString;

fn main() -> Result<(), git2::Error> {

    // Get path to git repo via command line args or assume current directory
    let repo_root: OsString = std::env::args_os()
                    .nth(1)
                    .unwrap_or_else(|| OsString::from("."));

    // Open git repo
    let repo = Repository::open(&repo_root).expect("Couldn't open repository");

    println!(
        "{:?}",
        repo.state()
    );

       // Get object database from the repo
    let odb = repo.odb().unwrap();

    // Loop through objects in db
    odb.foreach(|oid| {
                println!("{}objects/{}",repo.path().display() ,oid);

        // Return true because the closure has to return a boolean
        true
    })
    .unwrap();

    Ok(())
}

