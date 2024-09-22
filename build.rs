use std::process::Command;

fn main() {
    Command::new("cargo").args(["update", "nostr-types"]);
}
