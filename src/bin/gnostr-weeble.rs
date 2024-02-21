#[allow(unused_imports)]
use reqwest::{get, Url, Error};
use std::process::Command;

fn main() -> Result<(), reqwest::Error> {

	#[allow(clippy::if_same_then_else)]
	let gnostr_weeble = if cfg!(target_os = "windows") {
		Command::new("cmd")
			.args(["/C", "gnostr-weeble || echo weeble"])
			.output()
			.expect("failed to execute process")
	} else if cfg!(target_os = "macos") {
		Command::new("sh")
                .arg("-c")
                .arg("gnostr-weeble 2>/tmp/gnostr-legit.log || echo weeble")
                .output()
                .expect("failed to execute process")
	} else if cfg!(target_os = "linux") {
		Command::new("sh")
                .arg("-c")
                .arg("gnostr-weeble 2>/tmp/gnostr-legit.log || echo weeble")
                .output()
                .expect("failed to execute process")
	} else {
		Command::new("sh")
                .arg("-c")
                .arg("gnostr-weeble 2>/tmp/gnostr-legit.log || echo weeble")
                .output()
                .expect("failed to execute process")
	};

	let weeble = String::from_utf8(gnostr_weeble.stdout)
		.map_err(|non_utf8| {
			String::from_utf8_lossy(non_utf8.as_bytes()).into_owned()
		})
		.unwrap();

	//assert_eq!(weeble.is_empty(), true); // a)
	//
	println!("weeble={}", weeble);

	#[allow(clippy::if_same_then_else)]
	let gnostr_wobble = if cfg!(target_os = "windows") {
		Command::new("cmd")
			.args(["/C", "gnostr-wobble"])
			.output()
			.expect("failed to execute process")
	} else if cfg!(target_os = "macos") {
		Command::new("sh")
			.arg("-c")
			.arg("gnostr-wobble || echo wobble")
			.output()
			.expect("failed to execute process")
	} else if cfg!(target_os = "linux") {
		Command::new("sh")
			.arg("-c")
			.arg("gnostr-wobble || echo wobble")
			.output()
			.expect("failed to execute process")
	} else {
		Command::new("sh")
			.arg("-c")
			.arg("gnostr-wobble || echo wobble")
			.output()
			.expect("failed to execute process")
	};

	let wobble = String::from_utf8(gnostr_wobble.stdout)
		.map_err(|non_utf8| {
			String::from_utf8_lossy(non_utf8.as_bytes()).into_owned()
		})
		.unwrap();
    println!("wobble={}", wobble);

	#[allow(clippy::if_same_then_else)]
	let gnostr_blockheight = if cfg!(target_os = "windows") {
		Command::new("cmd")
			.args(["/C", "gnostr-blockheight"])
			.output()
			.expect("failed to execute process")
	} else if cfg!(target_os = "macos") {
		Command::new("sh")
			.arg("-c")
			.arg("gnostr-blockheight || echo blockheight")
			.output()
			.expect("failed to execute process")
	} else if cfg!(target_os = "linux") {
		Command::new("sh")
			.arg("-c")
			.arg("gnostr-blockheight || echo blockheight")
			.output()
			.expect("failed to execute process")
	} else {
		Command::new("sh")
			.arg("-c")
			.arg("gnostr-blockheight || echo blockheight")
			.output()
			.expect("failed to execute process")
	};

	let blockheight = String::from_utf8(gnostr_blockheight.stdout)
		.map_err(|non_utf8| {
			String::from_utf8_lossy(non_utf8.as_bytes()).into_owned()
		})
		.unwrap();
	println!("blockheight={}", blockheight);

    Ok(())
}
