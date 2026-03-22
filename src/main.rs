mod statusline_payload;

use statusline_payload::StatuslinePayload;
use std::io;
use std::path::Path;

fn main() {
    let payload: StatuslinePayload = serde_json::from_reader(io::stdin()).expect("failed to parse JSON");
    let current_dir = Path::new(&payload.workspace.current_dir);
    let dir_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let project_dir = Path::new(&payload.workspace.project_dir);
    let project_name = project_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let repo = git2::Repository::discover(&payload.workspace.current_dir).expect("failed to discover repository");
    let head = repo.head().expect("failed to get HEAD");
    let branch_name = head.shorthand().unwrap_or("HEAD");
    println!("{} {} {}", project_name, dir_name, branch_name);
}
