#[allow(unused_imports)]
use reqwest::{get, Url, Error};
use std::process::Command;
use std::process;

fn check_curl(){

    //println!("check_curl");

}

fn main() -> Result<(), reqwest::Error> {

    let now = chrono::Utc::now().timestamp();
    //println!("{}", now);

    check_curl();
	#[allow(clippy::if_same_then_else)]
	let blockchain_gnostr_weeble = if cfg!(target_os = "windows") {
		Command::new("cmd")
			.args(["/C", "gnostr-weeble || echo weeble"])
			.output()
			.expect("failed to execute process")
	} else if cfg!(target_os = "macos") {
		Command::new("curl")
                .arg("https://blockchain.info/q/getblockcount")
                .output()
                .expect("failed to execute process")
	} else if cfg!(target_os = "linux") {
		Command::new("curl")
                .arg("https://blockchain.info/q/getblockcount")
                .output()
                .expect("failed to execute process")
	} else {
		Command::new("curl")
                .arg("https://blockchain.info/q/getblockcount")
                .output()
                .expect("failed to execute process")
	};

	let blockchain_weeble = String::from_utf8(blockchain_gnostr_weeble.stdout)
		.map_err(|non_utf8| {
			String::from_utf8_lossy(non_utf8.as_bytes()).into_owned()
		})
		.unwrap();

	//assert_eq!(weeble.is_empty(), true); // a)
	//
	let mempool_gnostr_weeble = if cfg!(target_os = "windows") {
		Command::new("cmd")
			.args(["/C", "gnostr-weeble || echo weeble"])
			.output()
			.expect("failed to execute process")
	} else if cfg!(target_os = "macos") {
		Command::new("curl")
                .arg("https://mempool.space/api/blocks/tip/height")
                .output()
                .expect("failed to execute process")
	} else if cfg!(target_os = "linux") {
		Command::new("curl")
                .arg("https://mempool.space/api/blocks/tip/height")
                .arg("https://blockchain.info/q/getblockcount")
                .output()
                .expect("failed to execute process")
	} else {
		Command::new("curl")
                .arg("https://mempool.space/api/blocks/tip/height")
                .arg("https://blockchain.info/q/getblockcount")
                .output()
                .expect("failed to execute process")
	};

	let mempool_weeble = String::from_utf8(mempool_gnostr_weeble.stdout)
		.map_err(|non_utf8| {
			String::from_utf8_lossy(non_utf8.as_bytes()).into_owned()
		})
		.unwrap();

	assert_eq!(blockchain_weeble.is_empty(), false);
	assert_eq!(mempool_weeble.is_empty(), false);
    //assert_eq!(blockchain_weeble, mempool_weeble);

    let blockchain_weeble : i64 = blockchain_weeble.trim().parse().unwrap();
    //println!("{}", blockchain_weeble + 1);
    let mempool_weeble : i64 = mempool_weeble.trim().parse().unwrap();
    //println!("{}", mempool_weeble + 1);

    //
    if mempool_weeble as u64 == blockchain_weeble as u64
    {

        println!("{}", now/mempool_weeble);
        process::exit(0);

    }
    if mempool_weeble as u64 >= blockchain_weeble as u64
    {
        println!("{}", now/mempool_weeble);
    }
    else
    {
        println!("{}", now/blockchain_weeble);
    }

    Ok(())
}
