use git2::{Repository, IndexAddOption};
//use std::env::args;
//use structopt::StructOpt;

use std::path::Path;
use std::process;

use std::ffi::OsString;

//#[derive(StructOpt)]
//struct Args {
//    #[structopt(short, long)]
//    verbose: bool,
//    rest: Vec<String>,
//}

fn main() -> Result<(), git2::Error> {
    //let args = Args::from_args();




    // Get path to git repo via command line args or assume current directory
    let repo_root: OsString = std::env::args_os()
                    .nth(1)
                    .unwrap_or_else(|| OsString::from("."));

    // Open git repo
    let repo = Repository::open(&repo_root).expect("Couldn't open repository");

    println!(
        "{} state={:?}",
        repo.path().display(),
        repo.state()
    );


    //let path = Path::new(".");
    ////println!("path.diplay()={}", path.display());
    //let repo = Repository::open(path)?;
    //let repo_discovered = Repository::discover_path(path, 10)?;
    //println!("repo.path().diplay()={}", repo.path().display());
    //let str_path = format!("{:?}", repo.path().display());


    //println!("{}", str_path().display().contains("modules"));

    //let mut index = repo.index()?;
    //println!("Adding '{:?}'", index);

    process::exit(0);
    //for path in &args.rest {
    //    if let Ok(status) = repo.status_file(path) {
    //        if status.contains(git2::Status::WT_MODIFIED) || status.contains(git2::Status::WT_NEW) {
    //            //println!("Adding '{}'", path.display());
    //            println!("Adding '{}'", path);
    //            //index.add_path(path, IndexAddOption::DEFAULT)?;
    //            //index.add_path(path)?;
    //        } else {
    //            if args.verbose {
    //                //println!("Ignoring '{}' (not modified)", path.display());
    //                println!("Ignoring '{}' (not modified)", path);
    //            }
    //        }
    //    } else {
    //        //println!("Error: could not get status for '{}'", path.display());
    //        println!("Error: could not get status for '{}'", path);
    //    }
    //}

    //index.write()?;

    Ok(())
}

