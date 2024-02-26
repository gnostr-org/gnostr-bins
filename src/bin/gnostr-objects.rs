use git2::{Repository, IndexAddOption};
use std::path::Path;
use std::process;
use std::ffi::OsString;
use std::env;
use git2::Oid;

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

pub fn oid_to_str(oid: &Oid) -> Result<String, &'static str> {
    // Use the format!("{:x}", oid) for full 40-character hex string.
    // For a shorter representation, use oid.short_id() which returns a Result<Buf, Error>
    // and needs further conversion to String.
    Ok(format!("{:#x?}", oid))
    //Ok(format!("{:}", oid))
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
                println!("{}",*oid);
                println!("{:?}",oid_to_str(oid));
                //format!("{:#x?}", oid)
                let s:String = "Hello, world!".chars()
                    .map(|x| match x {
                        '!' => '?',
                        'A'..='Z' => 'X',
                        'a'..='z' => 'x',
                        _ => x
                    }).collect();
                println!("{}", s);// Xxxxx, xxxxx
                let s:String = oid_to_str(oid).expect("REASON");
                let s:String = s.chars()
                    .map(|x| match x {
                        '!' => '?',
                        'A'..='Z' => 'X',
                        'a'..='z' => 'x',
                        _ => x
                    }).collect();
                println!("{}", s);// Xxxxx, xxxxx
                //let first_two_chars: String = format!("{:?}",oid_to_str(oid));
                //let my_string = String::replace(first_two_chars, ['0', '3'], "");
                let mut result = str::replace("Hello World!", "!", "?");
                // Equivalently:
                result = "Hello World!".replace("!", "?");
                println!("{}", result); // => "Hello World?"
                //println!("1:First two characters: {}", first_two_chars);
                //let first_two_chars: String = first_two_chars.chars().take(2).collect();
                //let first_two_chars = &first_two_chars[..2];
                //println!("2:First two characters: {}", first_two_chars);
                //println!("{}objects/{}",repo.path().display() ,oid);
                //let (first_char, remainder) = car_cdr(oid_to_str(oid));
                //let (first_char, remainder) = car_cdr(oid_to_str(oid));
                //println!("first char: {}\n", first_char);
                //println!("first char: {}\nremainder: {}", first_char, remainder);
                //let my_string = String::from("Hello, world!");
                //let first_two_chars = &my_string[..2];
                //println!("First two characters: {}", first_two_chars);

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

