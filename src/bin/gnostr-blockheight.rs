use reqwest::Url;
use std::io::Read;
use gnostr_bins::{get_blockheight};

//fn mp_reqwest(){
//
//    let url = Url::parse("https://mempool.space/api/blocks/tip/height").unwrap();
//    let mut res = reqwest::blocking::get(url).unwrap();
//
//    let mut body = String::new();
//    res.read_to_string(&mut body).unwrap();
//
//    //println!("{}", body);
//
//}
fn main() {

  let bh = get_blockheight();
  println!("{}", bh.unwrap().to_string());

}
