extern crate getopts;
use std::env;
use getopts::Options;
//use git2::{Repository, Revwalk, Commit};
use git2::{Repository, Commit};

//use std::process;

#[allow(dead_code)]
fn print_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub fn main() -> Result<(), git2::Error> {


// Collect and parse arguments
let args: Vec<String> = env::args().collect();
let mut opts = Options::new();
opts.optopt("r", "ref", "Specify the Git reference (default: HEAD)", "REF");
opts.optopt("n", "number", "Specify the maximum number of commits to show (default: 10)", "NUMBER");

let matches = opts.parse(&args[1..]).unwrap();

// Extract and validate arguments
let _ref_name = matches.opt_str("r").unwrap_or("HEAD".to_string());
let num_commits = matches.opt_str("n")
    .unwrap_or("10".to_string());
let num_commits = num_commits.parse::<i32>().unwrap_or(10);

//println!("{}", &num_commits);
//println!("{}", print_type_of(&num_commits));
//println!("ref_name={:?}", _ref_name);

if num_commits <= 0 {
    return Err(
        git2::Error::from_str("Number of commits must be positive"));
}


  let repo = match Repository::open(".") {
    Ok(repo) => repo,
    Err(e) => panic!("Error opening repository: {}", e),
};

let mut revwalk = repo.revwalk()?;

revwalk.push_head()?;
revwalk.set_sorting(git2::Sort::TIME)?;


for rev in revwalk {
    let commit = repo.find_commit(rev?)?;
    let message = commit.summary_bytes().unwrap_or_else(|| commit.message_bytes());

    //We pad each git commit hash to be used in a sha256 context
    //NOTE: git will eventually have sha256 commit hashes

    //TODO: add Mode switch for hash list with or without message
    //println!("{:0>64}", commit.id());
    println!("{:0>64}\n{}", commit.id(), String::from_utf8_lossy(message));
}

    Ok(())
}

#[allow(dead_code)]
fn format_commit(_commit: &Commit, _format_str: &str, _show_date: bool, _show_message: bool) -> String {
    // This function is not implemented in this example, but provides a placeholder for future enhancements
    // It would allow more flexible output formatting based on the provided format string and boolean flags.
    panic!("Formatting not implemented.");
}

