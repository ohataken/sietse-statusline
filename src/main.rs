mod output_token;
mod statusline_payload;

use output_token::OutputToken;
use statusline_payload::StatuslinePayload;
use std::io;
use std::path::Path;

fn parse_args(args: impl Iterator<Item = String>) -> Vec<OutputToken> {
    args.map(|arg| match arg.as_str() {
        "--current-dir-name" => OutputToken::CurrentDirName,
        "--project-dir-name" => OutputToken::ProjectDirName,
        "--branch-name" => OutputToken::BranchName,
        other => {
            eprintln!("unknown argument: {}", other);
            std::process::exit(1);
        }
    })
    .collect()
}

fn build_output(
    tokens: &[OutputToken],
    project_dir_name: &str,
    current_dir_name: &str,
    branch_name: Option<&str>,
) -> String {
    let mut output = String::new();
    for token in tokens {
        match token {
            OutputToken::ProjectDirName => output.push_str(project_dir_name),
            OutputToken::CurrentDirName => output.push_str(current_dir_name),
            OutputToken::BranchName => {
                output.push_str(branch_name.unwrap_or("HEAD"));
            }
        }
    }
    output
}

fn main() {
    let payload: StatuslinePayload = serde_json::from_reader(io::stdin()).expect("failed to parse JSON");
    let current_dir = Path::new(&payload.workspace.current_dir);
    let current_dir_name = current_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let project_dir = Path::new(&payload.workspace.project_dir);
    let project_dir_name = project_dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let repo = git2::Repository::discover(&payload.workspace.current_dir).expect("failed to discover repository");
    let head = repo.head().expect("failed to get HEAD");
    let branch_name = head.shorthand().unwrap_or("HEAD");
    println!("{} {} {}", project_dir_name, current_dir_name, branch_name);
}
