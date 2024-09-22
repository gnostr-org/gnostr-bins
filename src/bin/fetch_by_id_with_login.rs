use gnostr_bins::{Command, Probe};
use nostr_types::{Filter, IdHex, RelayMessage};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let _ = args.next(); // program name
    let relay_url = match args.next() {
        Some(u) => u,
        None => panic!("Usage: fetch_by_id_with_login <RelayURL> <IdHex>"),
    };
    let id: IdHex = match args.next() {
        Some(id) => IdHex::try_from_str(&id)?,
        None => panic!("Usage: fetch_by_id_with_login <RelayURL> <IdHex>"),
    };

    let signer = gnostr_bins::load_signer()?;

    let (to_probe, from_main) = tokio::sync::mpsc::channel::<Command>(100);
    let (to_main, from_probe) = tokio::sync::mpsc::channel::<RelayMessage>(100);
    let inner_relay_url = relay_url.clone();
    let join_handle = tokio::spawn(async move {
        let mut probe = Probe::new(from_main, to_main);
        if let Err(e) = probe.connect_and_listen(&inner_relay_url).await {
            eprintln!("{}", e);
        }
    });

    let mut filter = Filter::new();
    filter.add_id(&id);

    gnostr_bins::req(&relay_url, signer, filter, to_probe, from_probe).await?;

    Ok(join_handle.await?)
}
