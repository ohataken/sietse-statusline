use crate::claude_argument_token::ClaudeArgumentToken;
use crate::statusline_payload::StatuslinePayload;
use std::path::Path;

pub fn eval(payload: &StatuslinePayload, tokens: Vec<ClaudeArgumentToken>) {
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

    let repo = git2::Repository::discover(&payload.workspace.current_dir)
        .expect("failed to discover repository");

    let head = repo.head().expect("failed to get HEAD");

    let branch_name = head.shorthand().unwrap_or("HEAD").to_string();

    let branch_head_sha = head
        .target()
        .expect("failed to get HEAD target")
        .to_string();

    for token in &tokens {
        match token {
            ClaudeArgumentToken::ProjectDirName => print!("{}", project_dir_name),
            ClaudeArgumentToken::CurrentDirName => print!("{}", current_dir_name),
            ClaudeArgumentToken::BranchName => print!("{}", branch_name),
            ClaudeArgumentToken::Black => print!("\x1b[30m"),
            ClaudeArgumentToken::Red => print!("\x1b[31m"),
            ClaudeArgumentToken::Green => print!("\x1b[32m"),
            ClaudeArgumentToken::Yellow => print!("\x1b[33m"),
            ClaudeArgumentToken::Blue => print!("\x1b[34m"),
            ClaudeArgumentToken::Magenta => print!("\x1b[35m"),
            ClaudeArgumentToken::Cyan => print!("\x1b[36m"),
            ClaudeArgumentToken::White => print!("\x1b[37m"),
            ClaudeArgumentToken::BrightBlack => print!("\x1b[90m"),
            ClaudeArgumentToken::BrightRed => print!("\x1b[91m"),
            ClaudeArgumentToken::BrightGreen => print!("\x1b[92m"),
            ClaudeArgumentToken::BrightYellow => print!("\x1b[93m"),
            ClaudeArgumentToken::BrightBlue => print!("\x1b[94m"),
            ClaudeArgumentToken::BrightMagenta => print!("\x1b[95m"),
            ClaudeArgumentToken::BrightCyan => print!("\x1b[96m"),
            ClaudeArgumentToken::BrightWhite => print!("\x1b[97m"),
            ClaudeArgumentToken::Reset => print!("\x1b[0m"),
            ClaudeArgumentToken::Space => print!(" "),
            ClaudeArgumentToken::Comma => print!(","),
            ClaudeArgumentToken::Slash => print!("/"),
            ClaudeArgumentToken::Hyphen => print!("-"),
            ClaudeArgumentToken::Underscore => print!("_"),
            ClaudeArgumentToken::Break => println!(),
            ClaudeArgumentToken::Worktree => {
                if repo.is_worktree() {
                    print!("worktree");
                }
            }
            ClaudeArgumentToken::BranchHeadSha => print!("{}", branch_head_sha),
        }
    }
}
