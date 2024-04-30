use futures::executor::block_on;
//use reqwest::into_url::IntoUrlSealed;
use url::Url;

fn parse_urls(urls_str: &str) -> Result<Vec<Url>, url::ParseError> {
let mut urls: Vec<Url> = Vec::new();
//
//    let mut part = String::new();
//    let mut collected = Vec::new();
//
//    let mut char_iter = urls_str.chars();
//
//
    for url_str in urls_str.chars() {
        //if urls_str.next() != Some('[') {
        //return ()
//    }
//    loop {
//        match urls_str.next()? {
//            ']' => {
//                collected.push(part);
//                return Some(collected)
//            }
//            ',' | ' ' => {
//                if !part.is_empty() {
//                    collected.push(part);
//                    part = String::new();
//                }
//            }
//            x => part.push(x),
//        }
//    }


        print!("LINE:8:{}\n", url_str);

        //let parsed_url = Url::parse(&url_str)?;
        //urls.push(parsed_url);
    }
    Ok(urls)
}

async fn print_relay_list() {
    let relay_list: &str = gnostr_bins::get_relays_public().unwrap().as_str();
    let vec_relay_list = parse_urls(gnostr_bins::get_relays_public().unwrap().as_str());
    //let vec_relay_list = parse_urls(gnostr_bins::get_relays_public().unwrap());
    //print!("{:#}", gnostr_bins::get_relays_public().unwrap());
}
fn main() {
    let future = print_relay_list(); // Nothing is printed
    block_on(future);
}
