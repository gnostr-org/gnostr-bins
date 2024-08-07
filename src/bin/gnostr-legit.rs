#![doc = include_str!("../../README.md")]
#![allow(unused)]
#![allow(dead_code)]
extern crate chrono;
//use std::time::SystemTime;
use std::any::type_name;
use std::convert::TryInto;
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{env, io, thread};

//use std::mem::size_of;
use argparse::{ArgumentParser, Store};
use chrono::offset::Utc;
use chrono::DateTime;
use git2::*;
use gitminer::Gitminer;
use pad::{Alignment, PadStr};
use sha2::{Digest, Sha256}; //for get_current_dir

extern crate gnostr_bins;
use gnostr_bins::{
    blockheight, get_blockheight, get_pwd, get_weeble, get_wobble, gitminer, post_event, repo,
    weeble, wobble, worker,
};
use gnostr_types::{ClientMessage, Event, Filter, RelayMessage, SubscriptionId};

//fn type_of<T>(_: T) -> &'static str {
//    type_name::<T>()
//}

const BITCOIN_GENESIS: &str = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f";

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn convert_to_u32(v: usize) -> Option<i8> {
    if v > (std::i8::MAX as i32).try_into().unwrap() {
        None
    } else {
        Some(v as i8)
    }
}

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

#[cfg(debug_assertions)]
fn example() {
    //println!("Debugging enabled");
    //println!("cwd={:?}",get_current_working_dir());
}

#[cfg(not(debug_assertions))]
fn example() {
    //println!("Debugging disabled");
    //println!("cwd={:?}",get_current_working_dir());
}

fn main() -> io::Result<()> {
    #[allow(clippy::if_same_then_else)]
    if cfg!(debug_assertions) {
        //println!("Debugging enabled");
    } else {
        //println!("Debugging disabled");
    }

    //println!("BTICOIN_GENESIS={}",BITCOIN_GENESIS);
    #[cfg(debug_assertions)]
    //println!("Debugging enabled");
    #[cfg(not(debug_assertions))]
    //println!("Debugging disabled");
    example();

    let start = time::get_time();
    let epoch = get_epoch_ms();
    //println!("{}", epoch);
    let system_time = SystemTime::now();

    let datetime: DateTime<Utc> = system_time.into();
    //println!("{}", datetime.format("%d/%m/%Y %T/%s"));
    //println!("{}", datetime.format("%d/%m/%Y %T"));

    //let cwd = get_current_working_dir();
    let cwd = get_pwd();
    //#[cfg(debug_assertions)]
    //println!("Debugging enabled");
    //println!("{:#?}", cwd);
    let state = repo::state();
    //println!("{:#?}", state);
    //
    let repo_root = std::env::args().nth(1).unwrap_or(".".to_string());
    //println!("repo_root={:?}", repo_root.as_str());
    let repo = Repository::open(repo_root.as_str()).expect("Couldn't open repository");
    //println!("{} state={:?}", repo.path().display(), repo.state());
    //println!("state={:?}", repo.state());

    //println!("clean {:?}", repo.state());
    #[allow(clippy::if_same_then_else)]
    let repo_path = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "cd"])
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg("pwd")
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("pwd")
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("pwd")
            .output()
            .expect("failed to execute process")
    };

    let path = String::from_utf8(repo_path.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    //println!("path={:?}", path);

    //#!/bin/bash
    //declare -a RELAYS
    //function gnostr-get-relays(){

    //RELAYS=$(curl  'https://api.nostr.watch/v1/online' 2>/dev/null |
    //    sed -e 's/[{}]/''/g' |
    //    sed -e 's/\[/''/g' |
    //    sed -e 's/\]/''/g' |
    //    sed -e 's/"//g' |
    //    awk -v k="text" '{n=split($0,a,","); for (i=1; i<=n; i++) print a[i]}')
    // 2>/dev/null

    //echo $RELAYS
    //}
    //gnostr-get-relays

    //#!/bin/bash
    //gnostr-git config --global --replace-all gnostr.relays "$(gnostr-get-relays)"
    // #&& git config -l | grep gnostr.relays

    //TODO:
    //fix this
    //git config --global --replace-all gnostr.relays $(gnostr-get-relays)

    //#[allow(clippy::if_same_then_else)]
    //let set_relays = if cfg!(target_os = "windows") {
    //    Command::new("cmd")
    //        .args(["/C", "gnostr-set-relays"])
    //        .output()
    //        .expect("try:\ngit config -l | grep gnostr.relays")
    //} else if cfg!(target_os = "macos") {
    //    Command::new("sh")
    //        .arg("-c")
    //        .arg("gnostr-set-relays")
    //        .output()
    //        .expect("try:\ngit config -l | grep gnostr.relays")
    //} else if cfg!(target_os = "linux") {
    //    Command::new("sh")
    //        .arg("-c")
    //        .arg("gnostr-set-relays")
    //        .output()
    //        .expect("try:\ngit config -l | grep gnostr.relays")
    //} else {
    //    Command::new("sh")
    //        .arg("-c")
    //        .arg("gnostr-set-relays")
    //        .output()
    //        .expect("try:\ngit config -l | grep gnostr.relays")
    //};

    let count = thread::available_parallelism()?.get();
    assert!(count >= 1_usize);
    //println!("{}={}", type_of(count), (count as i32));
    //println!("{}={}", type_of(count), (count as i64));
    //let mut hasher = Sha256::new();
    //hasher.update(pwd);
    //// `update` can be called repeatedly and is generic over `AsRef<[u8]>`
    //hasher.update("String data");
    //// Note that calling `finalize()` consumes hasher
    //let hash = hasher.finalize();
    ////println!("Binary hash: {:?}", hash);
    //println!("hash: {:?}", hash);
    //println!("sha256 before write: {:x}", hash);
    //println!("sha256 before write: {:X}", hash);

    let now = SystemTime::now();

    //// we sleep for 2 seconds
    //sleep(Duration::new(2, 0));
    // match now.elapsed() {
    //    Ok(elapsed) => {
    //        // it prints '2'
    //        println!("{}", elapsed.as_secs());
    //    }
    //    Err(e) => {
    //        // an error occurred!
    //        println!("Error: {e:?}");
    //    }
    //}

    #[allow(clippy::if_same_then_else)]
    let get_pwd = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo %cd%"])
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg("echo ${PWD##*/}")
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("echo ${PWD##*/}")
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo ${PWD##*/}")
            .output()
            .expect("failed to execute process")
    };

    let pwd = String::from_utf8(get_pwd.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    //println!("pwd={}", pwd);
    let mut hasher = Sha256::new();
    hasher.update(pwd.clone());
    //sha256sum <(echo gnostr-legit)
    let pwd_hash: String = format!("{:x}", hasher.finalize());
    //println!("pwd_hash={:?}", pwd_hash);

    let path = env::current_dir()?;

    //println!("The current directory is {}", path.display());
    #[cfg(debug_assertions)]
    println!("256:{}", gnostr_bins::get_weeble().unwrap().to_string());
    #[cfg(debug_assertions)]
    println!("257:{}", gnostr_bins::get_wobble().unwrap().to_string());
    #[cfg(debug_assertions)]
    println!(
        "258:{}",
        gnostr_bins::get_blockheight().unwrap().to_string()
    );
    #[cfg(debug_assertions)]
    println!(
        "{}/{}/{}/{}",
        gnostr_bins::get_pwd().unwrap().to_string(),
        gnostr_bins::get_weeble().unwrap().to_string(),
        gnostr_bins::get_blockheight().unwrap().to_string(),
        gnostr_bins::get_wobble().unwrap().to_string()
    );

    //TODO gnostr_event reuse weeble/blockheight/wobble
    let mut opts = gitminer::Options {
        threads: count.try_into().unwrap(),
        target: "00000".to_string(), //default 00000
        //gnostr:##:nonce
        //part of the gnostr protocol
        //src/worker.rs adds the nonce
        pwd_hash: pwd_hash.clone(),
        message: cwd.unwrap(),
        //message: message,
        //message: count.to_string(),
        //repo:    ".".to_string(),
        repo: path.as_path().display().to_string(),
        timestamp: time::now(),
        weeble: gnostr_bins::get_weeble().unwrap().to_string(),
        wobble: gnostr_bins::get_wobble().unwrap().to_string(),
        blockheight: gnostr_bins::get_blockheight().unwrap().to_string(),
        //.duration_since(SystemTime::UNIX_EPOCH)
    };

    parse_args_or_exit(&mut opts);

    let mut miner = match Gitminer::new(opts) {
        Ok(m) => m,
        Err(e) => {
            panic!("Failed to start git miner: {}", e);
        }
    };

    let hash = match miner.mine() {
        Ok(s) => s,
        Err(e) => {
            panic!("Failed to generate commit: {}", e);
        }
    };

    let mut hasher = Sha256::new();
    hasher.update(&hash);
    // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
    //hasher.update("String data");
    // Note that calling `finalize()` consumes hasher
    //let gnostr_sec = hasher.finalize();
    let gnostr_sec: String = format!("{:X}", hasher.finalize());
    //println!("Binary hash: {:?}", hash);
    //println!("hash before: {:?}", hash);
    //println!("hash after pad: {:?}", hash);
    //println!("&hash before: {:?}", &hash);
    //println!("&hash after pad: {:?}", &hash);
    //println!("gnostr_sec before pad: {:?}", gnostr_sec);
    //println!("gnostr_sec after pad: {:?}", gnostr_sec.pad(64, '0',
    // Alignment::Right, true)); println!("&gnostr_sec before pad: {:?}",
    // &gnostr_sec); println!("&gnostr_sec after pad: {:?}", &gnostr_sec.pad(64,
    // '0', Alignment::Right, true));

    //let s = "12345".pad(64, '0', Alignment::Right, true);
    //println!("s: {:?}", s);
    // echo "000000b64a065760e5441bf47f0571cb690b28fc" | openssl dgst -sha256 | sed
    // 's/SHA2-256(stdin)= //g'
    //
    //
    //shell test
    let touch = Command::new("sh")
        .args(["-c", "touch ", &hash])
        .output()
        .expect("failed to execute process");
    let touch_event = String::from_utf8(touch.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    let cat = Command::new("sh")
        .args(["-c", "touch ", &hash])
        .output()
        .expect("failed to execute process");
    let cat_event = String::from_utf8(cat.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    //shell test
    //git rev-parse --verify HEAD
    #[allow(clippy::if_same_then_else)]
    //println!("349:{:?}", gnostr_bins::get_weeble());
    //println!("350:{:?}", gnostr_bins::get_wobble());
    //println!("351:{:?}", gnostr_bins::get_blockheight());
    let event = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args([
                "/C",
                "gnostr --sec $(gnostr-sha256 $(gnostr-weeble || echo)) -t gnostr --tag weeble \
                 $(gnostr-weeble || echo weeble) --tag wobble $(gnostr-wobble || echo wobble) \
                 --tag blockheight $(gnostr-blockheight || echo blockheight) --content \"$(git \
                 diff HEAD~1 || git diff)\" ",
            ])
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .args([
                "-c",
                "gnostr --sec $(gnostr-sha256 $(gnostr-weeble || echo)) -t gnostr --tag weeble \
                 $(gnostr-weeble || echo weeble) --tag wobble $(gnostr-wobble || echo wobble) \
                 --tag blockheight $(gnostr-blockheight || echo blockheight) --content \"$(git \
                 diff HEAD~1 || git diff)\" ",
            ])
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "linux") {
        Command::new("sh")
            .args([
                "-c",
                "gnostr --sec $(gnostr-sha256 $(gnostr-weeble || echo)) -t gnostr --tag weeble \
                 $(gnostr-weeble || echo weeble) --tag wobble $(gnostr-wobble || echo wobble) \
                 --tag blockheight $(gnostr-blockheight || echo blockheight) --content \"$(git \
                 diff HEAD~1 || git diff)\" ",
            ])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .args([
                "-c",
                "gnostr --sec $(gnostr-sha256 $(gnostr-weeble || echo)) -t gnostr --tag weeble \
                 $(gnostr-weeble || echo weeble) --tag wobble $(gnostr-wobble || echo wobble) \
                 --tag blockheight $(gnostr-blockheight || echo blockheight) --content \"$(git \
                 diff HEAD~1 || git diff)\" ",
            ])
            .output()
            .expect("failed to execute process")
    };

    let gnostr_event = String::from_utf8(event.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();

    //assert...
    //echo gnostr|openssl dgst -sha256 | sed 's/SHA2-256(stdin)= //g'

    //gnostr-legit must only return a sha256 generated by the
    //recent commit hash
    //to enable nested commands
    //REF:
    //gnostr --hash $(gnostr legit . -p 00000 -m "git rev-parse --verify HEAD")
    //gnostr --sec $(gnostr --hash $(gnostr legit . -p 00000 -m "git rev-parse
    // --verify HEAD")) Example:
    //gnostr --sec $(gnostr --hash $(gnostr legit . -p 00000 -m "#gnostr will
    // exist!")) --envelope --content "$(git log -n 1)" | gnostr-cat -u
    // wss://relay.damus.io
    //
    //
    //
    let duration = time::get_time() - start;
    //println!("Success! Generated commit {} in {} seconds", hash,
    // duration.num_seconds());
    //
    //
    //let relay_url = "wss://nos.lol";
    //let event: Event = serde_json::from_str(&gnostr_event).unwrap();
    //gnostr_bins::post_event(relay_url, event);

    println!("{}", gnostr_event);
    Ok(())
}

fn parse_args_or_exit(opts: &mut gitminer::Options) {
    let mut ap = ArgumentParser::new();
    ap.set_description("Generate git commit sha with a custom prefix");
    ap.stop_on_first_argument(false);

    //ap.refer(&mut opts.repo)
    //    //.add_argument("repository-path", Store, "Path to your git repository
    // (required)");    .add_argument("repository-path", Store, "Path to your
    // git repository");    //.required();
    ap.refer(&mut opts.repo)
        .add_argument("repository-path", Store, "Path to your git repository");

    ap.refer(&mut opts.target).add_option(
        &["-p", "--prefix"],
        Store,
        "Desired commit prefix (required)",
    );
    //.required();

    ap.refer(&mut opts.threads).add_option(
        &["-t", "--threads"],
        Store,
        "Number of worker threads to use (default 8)",
    );

    ap.refer(&mut opts.message).add_option(
        &["-m", "--message"],
        Store,
        "Commit message to use (required)",
    );
    //.required();

    //ap.refer(&mut opts.timestamp)
    //    .add_option(&["--timestamp"], Store, "Commit timestamp to use (default
    // now)");

    ap.parse_args_or_exit();
}
