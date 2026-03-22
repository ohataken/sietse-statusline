mod statusline_payload;

use statusline_payload::StatuslinePayload;
use std::io;

fn main() {
    let payload: StatuslinePayload = serde_json::from_reader(io::stdin()).expect("failed to parse JSON");
    let repo = git2::Repository::discover(&payload.workspace.current_dir).expect("failed to discover repository");
    let head = repo.head().expect("failed to get HEAD");
    let branch_name = head.shorthand().unwrap_or("HEAD");
    println!("{}", branch_name);
}
