mod claude_argument_eval;
mod claude_argument_parser;
mod claude_argument_token;
mod statusline_payload;

use statusline_payload::StatuslinePayload;
use std::io;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let payload: StatuslinePayload =
        serde_json::from_reader(io::stdin()).expect("failed to parse JSON");

    let tokens = match args.get(1).map(|s| s.as_str()) {
        Some("claude") => claude_argument_parser::parse(&args[2..]),
        Some(other) => {
            eprintln!("unknown subcommand: {}", other);
            std::process::exit(1);
        }
        None => vec![],
    };

    claude_argument_eval::eval(&payload, tokens);
}
