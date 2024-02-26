use git2::{Repository, IndexAddOption};
use std::path::Path;
use std::process;
use std::ffi::OsString;
use std::env;

fn car_cdr(s: &str) -> (&str, &str) {
    for i in 1..5 {
        let r = s.get(0..i);
        match r {
            Some(x) => return (x, &s[i..]),
            None => (),
        }
    }

    (&s[0..0], s)
}

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

    let cwd = env::current_dir().unwrap();
let my_str: String = cwd.as_os_str().to_str().unwrap().to_string();
println!("{:?}", my_str);

let (first_char, remainder) = car_cdr(&my_str);
println!("first char: {}\n", first_char);
println!("first char: {}\nremainder: {}", first_char, remainder);
let (first_char, remainder) = car_cdr("test");
println!("first char: {}\nremainder: {}", first_char, remainder);

    Ok(())
}

