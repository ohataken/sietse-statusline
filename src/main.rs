mod statusline_payload;

use statusline_payload::StatuslinePayload;
use std::io;

fn main() {
    let payload: StatuslinePayload = serde_json::from_reader(io::stdin()).expect("failed to parse JSON");
    println!("sietse-statusline");
}
