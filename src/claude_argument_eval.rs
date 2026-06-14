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

    let repo = git2::Repository::discover(&payload.workspace.current_dir).ok();

    let head = repo.as_ref().and_then(|r| r.head().ok());

    let branch_name = head
        .as_ref()
        .map(|h| h.shorthand().unwrap_or("HEAD").to_string())
        .unwrap_or_default();

    let branch_head_sha = head
        .as_ref()
        .and_then(|h| h.target())
        .map(|oid| oid.to_string())
        .unwrap_or_default();

    let ahead_behind = repo
        .as_ref()
        .and_then(|r| {
            let local = head.as_ref()?.target()?;
            let shorthand = head.as_ref()?.shorthand()?;
            let branch = r.find_branch(shorthand, git2::BranchType::Local).ok()?;
            let upstream = branch.upstream().ok()?;
            let upstream_oid = upstream.get().target()?;
            r.graph_ahead_behind(local, upstream_oid).ok()
        })
        .unwrap_or((0, 0));
    let git_status = GitStatus::new(repo.as_ref(), ahead_behind);

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
                if repo.as_ref().is_some_and(|r| r.is_worktree()) {
                    print!("worktree");
                }
            }
            ClaudeArgumentToken::BranchHeadSha => print!("{}", branch_head_sha),
            ClaudeArgumentToken::ModelId => print!("{}", payload.model.id),
            ClaudeArgumentToken::ModelDisplayName => print!("{}", payload.model.display_name),
            ClaudeArgumentToken::GitStatus => print!("{}", git_status.symbols()),
            ClaudeArgumentToken::Bold => print!("\x1b[1m"),
            ClaudeArgumentToken::Literal(s) => print!("{}", s),
        }
    }
}

struct GitStatus {
    conflicted: bool,
    ahead: bool,
    behind: bool,
    untracked: bool,
    stashed: bool,
    unstaged: bool,
    staged: bool,
    renamed: bool,
    deleted: bool,
}

impl GitStatus {
    fn new(repo: Option<&git2::Repository>, ahead_behind: (usize, usize)) -> Self {
        let flags: Vec<git2::Status> = repo
            .and_then(|r| {
                let mut opts = git2::StatusOptions::new();
                opts.include_untracked(true)
                    .renames_head_to_index(true)
                    .renames_index_to_workdir(true)
                    .renames_from_rewrites(true);
                r.statuses(Some(&mut opts)).ok()
            })
            .map(|statuses| statuses.iter().map(|e| e.status()).collect())
            .unwrap_or_default();
        let any = |mask: git2::Status| flags.iter().any(|s| s.intersects(mask));
        let (ahead, behind) = ahead_behind;

        Self {
            conflicted: any(git2::Status::CONFLICTED),
            ahead: ahead > 0,
            behind: behind > 0,
            untracked: any(git2::Status::WT_NEW),
            stashed: repo.is_some_and(|r| r.find_reference("refs/stash").is_ok()),
            unstaged: any(git2::Status::WT_MODIFIED),
            staged: any(git2::Status::INDEX_NEW
                | git2::Status::INDEX_MODIFIED
                | git2::Status::INDEX_TYPECHANGE),
            renamed: any(git2::Status::INDEX_RENAMED | git2::Status::WT_RENAMED),
            deleted: any(git2::Status::INDEX_DELETED | git2::Status::WT_DELETED),
        }
    }

    fn symbols(&self) -> String {
        let mut status = String::new();

        if self.conflicted {
            status.push('=');
        }
        if self.ahead && self.behind {
            status.push('⇕');
        }
        if self.ahead && !self.behind {
            status.push('⇡');
        }
        if !self.ahead && self.behind {
            status.push('⇣');
        }
        if self.untracked {
            status.push('?');
        }
        if self.stashed {
            status.push('$');
        }
        if self.unstaged {
            status.push('!');
        }
        if self.staged {
            status.push('+');
        }
        if self.renamed {
            status.push('»');
        }
        if self.deleted {
            status.push('✘');
        }

        status
    }
}
