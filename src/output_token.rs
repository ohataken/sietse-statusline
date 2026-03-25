pub enum OutputToken {
    CurrentDirName,
    ProjectDirName,
    BranchName,
    AnsiColor(&'static str),
}
