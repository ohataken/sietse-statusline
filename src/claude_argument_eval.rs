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

    let claude_config_dir = std::env::var("CLAUDE_CONFIG_DIR").unwrap_or_default();
    let claude_config_dir_name = Path::new(&claude_config_dir)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    let repo = git2::Repository::discover(&payload.workspace.current_dir).ok();

    for token in &tokens {
        match token {
            ClaudeArgumentToken::ProjectDir => print!("{}", payload.workspace.project_dir),
            ClaudeArgumentToken::ProjectDirName => print!("{}", project_dir_name),
            ClaudeArgumentToken::CurrentDir => print!("{}", payload.workspace.current_dir),
            ClaudeArgumentToken::CurrentDirName => print!("{}", current_dir_name),
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
            ClaudeArgumentToken::ModelId => print!("{}", payload.model.id),
            ClaudeArgumentToken::ModelDisplayName => print!("{}", payload.model.display_name),
            ClaudeArgumentToken::SessionId => print!("{}", payload.session_id),
            ClaudeArgumentToken::ClaudeConfigDir => print!("{}", claude_config_dir),
            ClaudeArgumentToken::ClaudeConfigDirName => print!("{}", claude_config_dir_name),
            ClaudeArgumentToken::ContextWindowTotalInputTokens => {
                print!("{}", payload.context_window.total_input_tokens)
            }
            ClaudeArgumentToken::ContextWindowTotalOutputTokens => {
                print!("{}", payload.context_window.total_output_tokens)
            }
            ClaudeArgumentToken::ContextWindowSize => {
                print!("{}", payload.context_window.context_window_size)
            }
            ClaudeArgumentToken::ContextWindowUsedPercentage => {
                print!("{}", payload.context_window.used_percentage.unwrap_or(0.0))
            }
            ClaudeArgumentToken::ContextWindowRemainingPercentage => {
                print!(
                    "{}",
                    payload.context_window.remaining_percentage.unwrap_or(100.0)
                )
            }
            ClaudeArgumentToken::Bold => print!("\x1b[1m"),
            ClaudeArgumentToken::Literal(s) => print!("{}", s),
        }
    }
}
