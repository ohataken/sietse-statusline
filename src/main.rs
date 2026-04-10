mod claude_argument_token;
mod statusline_payload;

use claude_argument_token::ClaudeArgumentToken;
use statusline_payload::StatuslinePayload;
use std::io;
use std::path::Path;

fn parse_args(args: impl Iterator<Item = String>) -> Vec<ClaudeArgumentToken> {
    args.map(|arg| match arg.as_str() {
        "--current-dir-name" => ClaudeArgumentToken::CurrentDirName,
        "--project-dir-name" => ClaudeArgumentToken::ProjectDirName,
        "--branch-name" => ClaudeArgumentToken::BranchName,
        "--black" => ClaudeArgumentToken::Black,
        "--red" => ClaudeArgumentToken::Red,
        "--green" => ClaudeArgumentToken::Green,
        "--yellow" => ClaudeArgumentToken::Yellow,
        "--blue" => ClaudeArgumentToken::Blue,
        "--magenta" => ClaudeArgumentToken::Magenta,
        "--cyan" => ClaudeArgumentToken::Cyan,
        "--white" => ClaudeArgumentToken::White,
        "--bright-black" => ClaudeArgumentToken::BrightBlack,
        "--bright-red" => ClaudeArgumentToken::BrightRed,
        "--bright-green" => ClaudeArgumentToken::BrightGreen,
        "--bright-yellow" => ClaudeArgumentToken::BrightYellow,
        "--bright-blue" => ClaudeArgumentToken::BrightBlue,
        "--bright-magenta" => ClaudeArgumentToken::BrightMagenta,
        "--bright-cyan" => ClaudeArgumentToken::BrightCyan,
        "--bright-white" => ClaudeArgumentToken::BrightWhite,
        "--reset" => ClaudeArgumentToken::Reset,
        "--space" => ClaudeArgumentToken::Space,
        "--comma" => ClaudeArgumentToken::Comma,
        "--slash" => ClaudeArgumentToken::Slash,
        "--hyphen" => ClaudeArgumentToken::Hyphen,
        "--underscore" => ClaudeArgumentToken::Underscore,
        other => {
            eprintln!("unknown argument: {}", other);
            std::process::exit(1);
        }
    })
    .collect()
}

fn build_output(
    tokens: &[ClaudeArgumentToken],
    project_dir_name: &str,
    current_dir_name: &str,
    branch_name: Option<&str>,
) -> String {
    let mut output = String::new();
    for token in tokens {
        match token {
            ClaudeArgumentToken::ProjectDirName => output.push_str(project_dir_name),
            ClaudeArgumentToken::CurrentDirName => output.push_str(current_dir_name),
            ClaudeArgumentToken::BranchName => {
                output.push_str(branch_name.unwrap_or("HEAD"));
            }
            ClaudeArgumentToken::Black => output.push_str("\x1b[30m"),
            ClaudeArgumentToken::Red => output.push_str("\x1b[31m"),
            ClaudeArgumentToken::Green => output.push_str("\x1b[32m"),
            ClaudeArgumentToken::Yellow => output.push_str("\x1b[33m"),
            ClaudeArgumentToken::Blue => output.push_str("\x1b[34m"),
            ClaudeArgumentToken::Magenta => output.push_str("\x1b[35m"),
            ClaudeArgumentToken::Cyan => output.push_str("\x1b[36m"),
            ClaudeArgumentToken::White => output.push_str("\x1b[37m"),
            ClaudeArgumentToken::BrightBlack => output.push_str("\x1b[90m"),
            ClaudeArgumentToken::BrightRed => output.push_str("\x1b[91m"),
            ClaudeArgumentToken::BrightGreen => output.push_str("\x1b[92m"),
            ClaudeArgumentToken::BrightYellow => output.push_str("\x1b[93m"),
            ClaudeArgumentToken::BrightBlue => output.push_str("\x1b[94m"),
            ClaudeArgumentToken::BrightMagenta => output.push_str("\x1b[95m"),
            ClaudeArgumentToken::BrightCyan => output.push_str("\x1b[96m"),
            ClaudeArgumentToken::BrightWhite => output.push_str("\x1b[97m"),
            ClaudeArgumentToken::Reset => output.push_str("\x1b[0m"),
            ClaudeArgumentToken::Space => output.push(' '),
            ClaudeArgumentToken::Comma => output.push(','),
            ClaudeArgumentToken::Slash => output.push('/'),
            ClaudeArgumentToken::Hyphen => output.push('-'),
            ClaudeArgumentToken::Underscore => output.push('_'),
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
        .any(|t| matches!(t, ClaudeArgumentToken::BranchName));
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
