
use http::Uri;
use nostr_types::{Event, Filter, IdHex};

mod internal;
use internal::*;

mod reflog_simple;
use reflog_simple::*;

pub fn hash_list_w_commit_message(){
  let _ = ref_hash_list_w_commit_message();
}

pub fn hash_list(){
  let _ = ref_hash_list();
}

pub fn hash_list_padded(){
  let _ = ref_hash_list_padded();
}

pub fn url_to_host_and_uri(url: &str) -> (String, Uri) {
    let uri: http::Uri = url.parse::<http::Uri>().expect("Could not parse url");
    let authority = uri.authority().expect("Has no hostname").as_str();
    let host = authority
        .find('@')
        .map(|idx| authority.split_at(idx + 1).1)
        .unwrap_or_else(|| authority);
    if host.is_empty() {
        panic!("URL has empty hostname");
    }
    (host.to_owned(), uri)
}

pub fn fetch_by_filter(url: &str, filter: Filter) -> Vec<Event> {
    let (host,uri) = url_to_host_and_uri(url);
    let wire = filters_to_wire(vec![filter]);
    fetch(host, uri, wire)
}

pub fn fetch_by_id(url: &str, id: IdHex) -> Option<Event> {
    let mut filter = Filter::new();
    filter.add_id(&id);
    let events = fetch_by_filter(url, filter);
    if events.is_empty() {
        None
    } else {
        Some(events[0].clone())
    }
}

pub fn post_event(url: &str, event: Event) {
    let (host,uri) = url_to_host_and_uri(url);
    let wire = event_to_wire(event);
    post(host, uri, wire)
}

pub fn print_event(event: &Event) {
    println!("{}", serde_json::to_string(event).expect("Cannot serialize event to JSON"));
}
use sha256::{digest};
use std::error::Error;
use std::process;

pub struct Config {
    pub query: String,
}

pub fn print_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

impl Config {

    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() == 1 {

            println!("{}", digest("".to_string()));
            process::exit(0);

        }

        let query = args[1].clone();
        Ok(Config { query })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("{}", digest(config.query));
    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
let mut results = Vec::new();
    for line in contents.lines() {
        // do something with line
        println!("{}", line);
        if line.contains(query) {
            // do something with line
            let val = digest(query);
            println!("{}", val);
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn one_query() {
        let query = digest("");
        let contents = "\
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

        assert_eq!(vec!["e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"], search(&query, contents));
    }
    #[test]
    #[should_panic]
    fn panic_query() {
        let query = digest("");
        let contents = "\
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 ";

        assert_eq!(vec!["e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"], search(&query, contents));
    }
}

//#[cfg(debug_assertions)]
//let cloned_query = query.clone();
//#[cfg(debug_assertions)]
//println!("{:?}", print_type_of(&query));
//#[cfg(debug_assertions)]
//println!("{:?}", print_type_of(&cloned_query));
//println!("{:?}", query);
//#[cfg(debug_assertions)]
//let s = &"hello world".to_string();
//#[cfg(debug_assertions)]
//let cloned_s = s.clone();
//#[cfg(debug_assertions)]
//println!("{:?}", print_type_of(&s));
//#[cfg(debug_assertions)]
//println!("{:?}", print_type_of(&cloned_s));
