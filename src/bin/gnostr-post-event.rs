use nostr_types::Event;
use std::env;
use std::io::Read;
use std::process;

fn main() {
    //!
    //! USAGE:
    //!
    //! gnostr --sec $(gnostr-sha256) --content 'test'  | gnostr-post-event wss://relay.damus.io
    //!
    //! gnostr --sec $(gnostr-sha256) --content "$(gnostr-git show HEAD)" | gnostr-post-event wss://relay.damus.io
    //!
    //! gnostr --sec $(gnostr-sha256) --content "$(gnostr-git-reflog -gd)" | gnostr-post-event wss://relay.damus.io
    //!

    //let v: Vec<u8> = vec![0, 1, 2, 3];
    //// The `Vec` type implements the `Index` trait so you can do:
    //println!("{:?}", v);
    //if v.len() == 0 {
    //    println!("v.len() = {}", 0);
    //};
    //if v.len() == 1 {
    //    println!("v.len() = {}", 1);
    //};
    //if v.len() == 2 {
    //    println!("v.len() = {}", 2);
    //};
    //if v.len() == 3 {
    //    println!("v.len() = {}", 3);
    //} else {
    //    println!("{}:{}", "zero", v[0]);
    //    println!("{}:{}", "one", v[1]);
    //    println!("{}:{}", "two", v[2]);
    //    println!("{}:{}", "three", v[3]);
    //    println!("v.len() = {}", v.len());
    //};

    let args_vector: Vec<String> = env::args().collect();
    println!("args_vector = {:?}", args_vector);
    println!("args_vector.len() = {:?}", args_vector.len());
    if args_vector.len() == 0 {
        println!("args_vector.len() = {}", 0);
    };
    if args_vector.len() == 1 {
        println!("args_vector.len() = {}", 1);
        let app: Vec<u8> = args_vector[0].clone().into();
        println!("app.len() = {:?}", app.len());
        println!("Searching for {:?}", app);
    };
    if args_vector.len() == 2 {
        println!("args_vector.len() = {}", 2);
        let app: Vec<u8> = args_vector[0].clone().into();
        println!("app.len() = {:?}", app.len());
        println!("Searching for {:?}", app);
        let relay: Vec<u8> = args_vector[1].clone().into();
        println!("relay.len() = {:?}", relay.len());
        println!("Searching for {:?}", relay);
    };
    if args_vector.len() == 3 {
        println!("args_vector.len() = {}", 2);
        let app: Vec<u8> = args_vector[0].clone().into();
        println!("app.len() = {:?}", app.len());
        println!("app = {:?}", app);
        let relay: Vec<u8> = args_vector[1].clone().into();
        println!("relay.len() = {:?}", relay.len());
        println!("relay = {:?}", relay);
        let content: Vec<u8> = args_vector[2].clone().into();
        println!("content.len() = {:?}", content.len());
        println!("content = {:?}", content);
    };
    //process::exit(0);

    #[allow(unreachable_code)]
    for i in 0..args_vector.len() {
        if i == args_vector.len() {
            //process::exit(0);
            //process::exit(1);
            process::exit(i.try_into().unwrap());
            //unsafe { libc::exit(1); }
        }
        println!("i={}", i);
        println!("args_vector[{}]={}", i, args_vector[i]);
    }

    let mut args = env::args();

    //    let _ = args.next(); // program name
    //    let relay_url = match args.next() {
    //        Some(u) => u,
    //        None => panic!("Usage:\ngnostr --sec $(gnostr-sha256) --content 'test'  | gnostr-post-event wss://relay.damus.io\ngnostr --sec $(gnostr-sha256) --content \"$(gnostr-git show HEAD)\ngnostr --sec $(gnostr-sha256) --content \"$(gnostr-git-reflog -gd)\" | gnostr-post-event wss://relay.damus.io | gnostr-post-event wss://relay.damus.io"),
    //    };
    //
    //    let mut s: String = String::new();
    //    std::io::stdin().read_to_string(&mut s).unwrap();
    //
    //    println!("{}", s);//TODO:write event to .gnostr/EVENT_HASH.event
    //
    //// TODO: detect { EVENT: } envelope
    //    let event: Event = serde_json::from_str(&s).unwrap();
    //
    //    gnostr_bins::post_event(&relay_url, event);
}
