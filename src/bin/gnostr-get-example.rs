use tokio::runtime::Runtime;

use std::{io::Read, time::Instant};

//use ureq::get;

const URL: &str = "https://mempool.space/api/blocks/tip/height";

fn main() {
    let n = 100;
    {
        let start = Instant::now();
        let res = blocking(n);
        println!("blocking {:?} {} bytes", start.elapsed(), res);
    }
    {
        let start = Instant::now();
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        let res = rt.block_on(non_blocking(n));
        println!("async    {:?} {} bytes", start.elapsed(), res);
    }
}

fn blocking(n: usize) -> usize {
    (0..n)
        .into_iter()
        .map(|_| {
            std::thread::spawn(|| {
                let mut body = ureq::get(URL).call().expect("REASON").into_reader();
                let mut buf = Vec::new();
                body.read_to_end(&mut buf).unwrap();
                // print block count from mempool.space or panic
                let text = match std::str::from_utf8(&buf) {
                    Ok(s) => s,
                    Err(_) => panic!("Invalid ASCII data"),
                };
                println!("{}", text);
                buf.len()
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .map(|it| it.join().unwrap())
        .sum()
}

async fn non_blocking(n: usize) -> usize {
    let tasks = (0..n)
        .into_iter()
        .map(|_| {
            tokio::spawn(async move {
                let body = reqwest::get(URL).await.unwrap().bytes();
                body.await.unwrap().len()
                // print block count from mempool.space or panic
                //let text = match std::str::from_utf8(&body) {
                //    Ok(s) => s,
                //    Err(_) => panic!("Invalid ASCII data"),
                //};
                //println!("{}", text);
            })
        })
        .collect::<Vec<_>>();

    let mut res = 0;
    for task in tasks {
        res += task.await.unwrap();
    }
    res
}
