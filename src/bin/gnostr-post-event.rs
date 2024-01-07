
use std::env;
use std::io::Read;
use nostr_types::Event;

fn main() {

// gnostr --sec $(gnostr-sha256) --content 'test'  | gnostr-post-event wss://relay.damus.io
// gnostr --sec $(gnostr-sha256) --content "$(gnostr-git show HEAD)" | gnostr-post-event wss://relay.damus.io
    let mut args = env::args();
    let _ = args.next(); // program name
    let relay_url = match args.next() {
        Some(u) => u,
        None => panic!("Usage:\ngnostr --sec $(gnostr-sha256) --content 'test'  | gnostr-post-event wss://relay.damus.io\ngnostr --sec $(gnostr-sha256) --content \"$(gnostr-git show HEAD)\" | gnostr-post-event wss://relay.damus.io"),
    };

    let mut s: String = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();

    println!("{}", s);//TODO:write event to .gnostr/EVENT_HASH.event

// TODO: detect { EVENT: } envelope
    let event: Event = serde_json::from_str(&s).unwrap();

    gnostr_bins::post_event(&relay_url, event);
}
