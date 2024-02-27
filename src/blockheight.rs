pub fn blockheight() -> Result<String, &'static str> {

use reqwest::Url;
use std::io::Read;

    let url = Url::parse("https://mempool.space/api/blocks/tip/height").unwrap();
    let mut res = reqwest::blocking::get(url).unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    Ok(format!("{}", body))

}
