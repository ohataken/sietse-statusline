use crate::claude_argument_token::ClaudeArgumentToken;

pub fn parse(args: &[String]) -> Vec<ClaudeArgumentToken> {
    args.iter()
        .map(|arg| match arg.as_str() {
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
            "--break" => ClaudeArgumentToken::Break,
            "--worktree" => ClaudeArgumentToken::Worktree,
            "--branch-head-sha" => ClaudeArgumentToken::BranchHeadSha,
            "--bold" => ClaudeArgumentToken::Bold,
            "--model-id" => ClaudeArgumentToken::ModelId,
            other => ClaudeArgumentToken::Literal(other.to_string()),
        })
        .collect()
}
