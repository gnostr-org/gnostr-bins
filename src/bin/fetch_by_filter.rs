use nostr_probe::{Command, ExitMessage, Probe};
use nostr_types::{Filter, SubscriptionId};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    let _ = args.next(); // program name
    let relay_url = match args.next() {
        Some(u) => u,
        None => panic!("Usage: fetch_by_filter <RelayURL> <FilterJSON>"),
    };
    let filter: Filter = match args.next() {
        Some(filter) => match serde_json::from_str(&filter) {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        },
        None => panic!("Usage: fetch_by_filter <RelayURL> <FilterJSON>"),
    };

    let (tx, rx) = tokio::sync::mpsc::channel::<Command>(100);

    let our_sub_id = SubscriptionId("fetch_by_filter".to_string());
    let cloned_sub_id = our_sub_id.clone();

    let join_handle = tokio::spawn(async move {
        let mut probe = Probe::new(
            rx,
            vec![
                ExitMessage::Eose(cloned_sub_id.clone()),
                ExitMessage::Closed(cloned_sub_id),
                ExitMessage::Notice,
            ],
        );

        if let Err(e) = probe.connect_and_listen(&relay_url).await {
            eprintln!("{}", e);
        }
    });

    tx.send(Command::FetchEvents(our_sub_id, vec![filter]))
        .await?;

    Ok(join_handle.await?)
}
