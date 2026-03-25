pub enum OutputToken {
    CurrentDirName,
    ProjectDirName,
    BranchName,
    AnsiColor(&'static str),
    Separator(&'static str),
}
