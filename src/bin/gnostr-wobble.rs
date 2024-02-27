use reqwest::Url;
use std::io::Read;
use gnostr_bins::{get_blockheight};

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {

  let start = SystemTime::now();
  let now = start
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards");


  let since_the_epoch =
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("get millis error");
    let seconds = since_the_epoch.as_secs();
    let subsec_millis = since_the_epoch.subsec_millis() as u64;
    let now_millis = seconds * 1000 + subsec_millis;
    //println!("now millis: {}", seconds * 1000 + subsec_millis);

  //let mut tmp_stringi = String::new();
  //now.read_to_string(&mut tmp_string).unwrap();
  //let tmp_u32 = tmp_string.parse::<u32>().unwrap_or(0);

  let _ = get_blockheight();
    let url = Url::parse("https://mempool.space/api/blocks/tip/height").unwrap();
    let mut res = reqwest::blocking::get(url).unwrap();

    let mut tmp_string = String::new();
    res.read_to_string(&mut tmp_string).unwrap();
    let tmp_u64 = tmp_string.parse::<u64>().unwrap_or(0);

    let wobble = now_millis as f64 % tmp_u64 as f64;
    println!("{}", format!("{}", wobble.floor()));
}
