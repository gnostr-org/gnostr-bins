use futures::executor::block_on;
//use reqwest::into_url::IntoUrlSealed;
use url::Url;

async fn parse_urls(urls_str: &str) -> Result<Vec<String>, url::ParseError> {
let mut urls: Vec<String> = Vec::new();
//
    let mut part = String::new();
    let mut collected = Vec::new();
//
      let mut char_iter = urls_str.chars();
//
//
    for url_str in urls_str.chars() {
        if char_iter.next() != Some('[') {
        //skip
        }
    loop {
        //match char_iter.next().ok_or(/* ... */)? {
        match char_iter.next() {
            //']' => {
	    Some(']') => {
                collected.push(part);
                //return std::result::Result::Ok(collected) as Vec<Url>
		//return Ok(std::result::Result::Ok(collected))
		return std::result::Result::Ok(collected)
                //return Result(collected)
//                return Ok(collected ) as Vec<Url>
                //return Ok(collected as Vec<Url>)
                //return Ok(collected as Vec<Url>)
                //return collected
            }
            //',' | ' ' => {
            Some(',') | Some(' ') => {
                if !part.is_empty() {
                    collected.push(part);
                    part = String::new();
                }
            }
            x => part.push(x.expect("REASON")),
//            x => part.push(x),
        }
    }


        //print!("LINE:8:{}\n", url_str);
        //println!("LINE:8:{:?}\n", urls);

        //let parsed_url = Url::parse(&url_str)?;
        //urls.push(parsed_url.to_string());
    }
    Ok(urls)
}

async fn print_relay_list() {
    let relay_list: &str = gnostr_bins::get_relays_public().unwrap().as_str();
    let vec_relay_list = parse_urls(&gnostr_bins::get_relays_public().unwrap().as_str()).await;
    //let vec_relay_list = parse_urls(gnostr_bins::get_relays_public().unwrap());
//    print!("{:}", format!("{}",vec_relay_list.unwrap()));
	println!("{:?}", vec_relay_list.unwrap());
    //print!("{:#}", gnostr_bins::get_relays_public().unwrap());
}
fn main() {
    let future = print_relay_list(); // Nothing is printed
    block_on(future);
}
