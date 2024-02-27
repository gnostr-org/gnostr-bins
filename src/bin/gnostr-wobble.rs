use reqwest::Url;
use std::io::Read;
use gnostr_bins::{get_blockheight};
use gnostr_bins::{get_weeble};
use gnostr_bins::{get_wobble};
fn main() {

  let get_bh = get_blockheight();
  //let mut bh_body = String::new();
  let mut bh_body = get_bh.clone();
  println!("{}", bh_body.unwrap().to_string());


  //let get_weeble = get_weeble();
  //let get_wobble = get_wobble();

  //let url = Url::parse("https://mempool.space/api/blocks/tip/height").unwrap();
  //let mut res = reqwest::blocking::get(url).unwrap();

  //let mut bh_body = String::new();
  //res.read_to_string(&mut bh_body).unwrap();
  //println!("{}", bh_body);

  //let mut weeblebody = String::new();
  //println!("{}", bh_body);
}
