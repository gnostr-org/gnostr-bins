extern crate getopts;
use std::env;
use getopts::Options;
//use git2::{Repository, Revwalk, Commit};
use git2::{Repository, Commit};

use std::process;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
    process::exit(0);
}

pub fn hash_list(_program: &str, _opts: &Options) -> Result<(), git2::Error> {
    //let brief = format!("Usage: {} FILE [options]", _program);
    //print!("hash_list_commit_message:\n{}", _opts.usage(&brief));
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("Error opening repository: {}", e),
    };
    
    let mut revwalk = repo.revwalk()?;
    
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;
    
    
    for rev in revwalk {
        let commit = repo.find_commit(rev?)?;
        println!("{:0>64}", commit.id());
    }
    Ok(())
       // process::exit(0);
}
pub fn hash_list_w_commit_message(_program: &str, _opts: &Options) -> Result<(), git2::Error> {
    //let brief = format!("Usage: {} FILE [options]", _program);
    //print!("hash_list_commit_message:\n{}", _opts.usage(&brief));
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
    
        println!("{:0>64}\n{}", commit.id(), String::from_utf8_lossy(message));
    }
    Ok(())
       // process::exit(0);
}

#[allow(dead_code)]
fn print_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub fn main() -> Result<(), git2::Error> {


// Collect and parse arguments
let args: Vec<String> = env::args().collect();
let program = args[0].clone();


let mut opts = Options::new();
//opts.optopt("o", "", "set output file name", "NAME");
//opts.optopt("r", "ref", "Specify the Git reference (default: HEAD)", "REF");
//opts.optopt("n", "number", "Specify the maximum number of commits to show (default: 10)", "NUMBER");
opts.optflag("h", "help", "print this help menu");
opts.optflag("m", "messages", "print reflog with commit messages");
let matches = match opts.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!("{}", f.to_string()) }
};
if matches.opt_present("h") {
    print_usage(&program, &opts);
    process::exit(0);
}
if matches.opt_present("m") {
    let _ = hash_list_w_commit_message(&program, &opts);
} else {
    let _ = hash_list(&program, &opts);
}
    Ok(())
}

#[allow(dead_code)]
fn format_commit(_commit: &Commit, _format_str: &str, _show_date: bool, _show_message: bool) -> String {
    // This function is not implemented in this example, but provides a placeholder for future enhancements
    // It would allow more flexible output formatting based on the provided format string and boolean flags.
    panic!("Formatting not implemented.");
}

