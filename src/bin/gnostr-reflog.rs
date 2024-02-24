extern crate getopts;
use getopts::Options;
use std::env;
//use git2::{Repository, Revwalk, Commit};
use git2::{Commit, Repository};
use std::io;
use ascii::AsciiChar;
use std::process;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
    process::exit(0);
}
fn sec(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("sec:\n{}", opts.usage(&brief));
    process::exit(0);
}
fn hash(program: &str, opts: &Options) {
    //hash a following value
    //TODO detect stream or file
    let brief = format!("Usage: {} [options]", program);
    print!("hash:\n{}", opts.usage(&brief));
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
        let message = commit
            .summary_bytes()
            .unwrap_or_else(|| commit.message_bytes());

        println!("{:0>64}\n{}", commit.id(), String::from_utf8_lossy(message));
    }
    Ok(())
    // process::exit(0);
}

fn do_work(inp: &str, out: Option<String>) {
    println!("{}", inp);
    match out {
        Some(x) => println!("{}", x),
        None => println!("No Output"),
    }
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
    opts.optopt("o", "output", "set output file name", "NAME");
    opts.optopt("r", "ref", "Specify the Git reference (default: HEAD)", "REF");
    opts.optopt("n", "number", "Specify the maximum number of commits to show (default: 10)", "NUMBER");

    opts.optopt("s", "sec", "use following privkey", "SEC");

    opts.optflag("h", "help", "print this help menu");
    opts.optflag("m", "messages", "print reflog with commit messages");

    //println!("args.len()={}",args.len());

    if args.len() == 1 {

        //println!("program={}",program);
        //println!("args[0]={}",args[0]);

        let mut input = String::new();
        //capture input from prompt
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        //check if hex
        let mut count = 0;
        let mut key_maybe = true;
        //REF: https://docs.rs/ascii/latest/ascii/enum.AsciiChar.html#method.is_ascii_hexdigit
        for (_i, c) in input.trim().chars().enumerate() {
            if c.is_ascii() {
            //TODO:more validation
            assert!(ascii::AsciiChar::Space.is_ascii_blank());
            assert!(ascii::AsciiChar::Space.is_ascii_blank());
            assert!(ascii::AsciiChar::Tab.is_ascii_blank());
            assert!(!ascii::AsciiChar::VT.is_ascii_blank());
            assert!(!ascii::AsciiChar::LineFeed.is_ascii_blank());
            assert!(!ascii::AsciiChar::CarriageReturn.is_ascii_blank());
            assert!(!ascii::AsciiChar::FF.is_ascii_blank());

            assert!(AsciiChar::Space.is_ascii_blank());
            assert!(AsciiChar::Tab.is_ascii_blank());
            assert!(!AsciiChar::VT.is_ascii_blank());
            assert!(!AsciiChar::LineFeed.is_ascii_blank());
            assert!(!AsciiChar::CarriageReturn.is_ascii_blank());
            assert!(!AsciiChar::FF.is_ascii_blank());

            #[cfg(debug_assertions)]
            if c.is_ascii_control() {
                println!("{}.is_asscii_control()={}",c,c.is_ascii_control());
            }
            #[cfg(debug_assertions)]
            if c.is_ascii() {
                println!("{}.is_asscii()={}",c,c.is_ascii());
            }
            #[cfg(debug_assertions)]
            if c.is_ascii_graphic() {
                println!("{}.is_asscii_graphic()={}",c,c.is_ascii_graphic());
            }
            #[cfg(debug_assertions)]
            if c.is_whitespace() {
                println!("{}.is_whitespace()={}",c,c.is_whitespace());
            }
            #[cfg(debug_assertions)]
            if c.is_ascii_whitespace() {
                println!("{}.is_ascii_whitespace()={}",c,c.is_ascii_whitespace());
            }
            #[cfg(debug_assertions)]
            if !c.is_ascii_hexdigit() {
                key_maybe = false;
            }
            #[cfg(debug_assertions)]
            println!("{}:{}={}", _i, c, c.is_ascii_hexdigit());
            #[cfg(debug_assertions)]
            println!("{}", c);
            count = count + 1;
            
        }//end is_ascii
        }//end for loop

        #[cfg(debug_assertions)]
        println!("count={:?}", count);
        #[cfg(debug_assertions)]
        println!("key_maybe={:?}", key_maybe);

        //we assume the input is a key
        //we assume it is a privkey for now
        if count == 64 && key_maybe == true {
            println!("handle key");
        } else {
            //println!("test for sub commands");
            if input.trim() == "input" {
                println!("input={}", input);
            }
            if input.trim() == "install" {
                println!("input={}", input);
            }
            let _ = hash_list(&program, &opts);
            process::exit(0);
        }//end else
    //end if args.len() == 1
    } else {
        //let _ = hash_list(&program, &opts);
        //print_usage(&program, &opts);
        //process::exit(0);
    }
    if args.len() > 1 {

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    //let output = matches.opt_str("o");
    ////let output = if matches.opt_present("o") {
    ////    //println!("output:0{:}", &args[0].to_string());
    ////    //println!("output:1{:}", &args[1].to_string());
    ////    //println!("output:2{:}", &args[2].to_string());
    ////    //println!("output:3{:}", &args[3].to_string());
    ////    //print_usage(&program, &opts);
    ////    //process::exit(0);
    ////} else {
    ////    print_usage(&program, &opts);
    ////};
    //let input = if !matches.free.is_empty() {
    //    //catch a free floating arg before or after -o <String>
    //    matches.free[0].clone().to_string();
    //    //println!("input:0{:}", &args[0].to_string());
    //    //println!("input:1{:}", &args[1].to_string());
    //    //println!("input:2{:}", &args[2].to_string());
    //    //println!("input:3{:}", &args[3].to_string());
    //} else {
    //    print_usage(&program, &opts);
    //};

    //println!("!====={} true", !matches.free.is_empty());
    //println!("====={} false", matches.free.is_empty());
    //println!("====={}", matches.free[0].clone().to_string());
    ////println!("====={}", matches.free[1].clone().to_string());
    //println!("input===={:?}", input);
    //println!("output==={:?}", output);
    ////do_work(&input, output);
    //process::exit(0);

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        process::exit(0);
    }
    if matches.opt_present("s") {
        sec(&program, &opts);
        process::exit(0);
    }
    if matches.opt_present("m") {
        let _ = hash_list_w_commit_message(&program, &opts);
    } else {
        let _ = hash_list_w_commit_message(&program, &opts);
        //let _ = hash_list(&program, &opts);
    }
    }

    Ok(())
}

#[allow(dead_code)]
fn format_commit(
    _commit: &Commit,
    _format_str: &str,
    _show_date: bool,
    _show_message: bool,
) -> String {
    // This function is not implemented in this example, but provides a placeholder for future enhancements
    // It would allow more flexible output formatting based on the provided format string and boolean flags.
    panic!("Formatting not implemented.");
}
