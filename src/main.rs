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
        "--black" => OutputToken::AnsiColor("\x1b[30m"),
        "--red" => OutputToken::AnsiColor("\x1b[31m"),
        "--green" => OutputToken::AnsiColor("\x1b[32m"),
        "--yellow" => OutputToken::AnsiColor("\x1b[33m"),
        "--blue" => OutputToken::AnsiColor("\x1b[34m"),
        "--magenta" => OutputToken::AnsiColor("\x1b[35m"),
        "--cyan" => OutputToken::AnsiColor("\x1b[36m"),
        "--white" => OutputToken::AnsiColor("\x1b[37m"),
        "--bright-black" => OutputToken::AnsiColor("\x1b[90m"),
        "--bright-red" => OutputToken::AnsiColor("\x1b[91m"),
        "--bright-green" => OutputToken::AnsiColor("\x1b[92m"),
        "--bright-yellow" => OutputToken::AnsiColor("\x1b[93m"),
        "--bright-blue" => OutputToken::AnsiColor("\x1b[94m"),
        "--bright-magenta" => OutputToken::AnsiColor("\x1b[95m"),
        "--bright-cyan" => OutputToken::AnsiColor("\x1b[96m"),
        "--bright-white" => OutputToken::AnsiColor("\x1b[97m"),
        "--reset" => OutputToken::AnsiColor("\x1b[0m"),
        "--space" => OutputToken::Separator(" "),
        "--comma" => OutputToken::Separator(","),
        "--slash" => OutputToken::Separator("/"),
        "--hyphen" => OutputToken::Separator("-"),
        "--underscore" => OutputToken::Separator("_"),
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
            OutputToken::AnsiColor(seq) => output.push_str(seq),
            OutputToken::Separator(sep) => output.push_str(sep),
        }
    }
    output
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let payload: StatuslinePayload =
        serde_json::from_reader(io::stdin()).expect("failed to parse JSON");
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

    let tokens = match args.get(1).map(|s| s.as_str()) {
        Some("claude") => parse_args(args.into_iter().skip(2)),
        Some(other) => {
            eprintln!("unknown subcommand: {}", other);
            std::process::exit(1);
        }
        None => vec![],
    };

    let needs_branch = tokens
        .iter()
        .any(|t| matches!(t, OutputToken::BranchName));
    let branch_name = if needs_branch {
        let repo = git2::Repository::discover(&payload.workspace.current_dir)
            .expect("failed to discover repository");
        let head = repo.head().expect("failed to get HEAD");
        Some(head.shorthand().unwrap_or("HEAD").to_string())
    } else {
        None
    };
    let output =
        build_output(&tokens, project_dir_name, current_dir_name, branch_name.as_deref());
    println!("{}", output);
}
