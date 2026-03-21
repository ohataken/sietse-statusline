use serde::Deserialize;

#[derive(Deserialize)]
pub struct StatuslinePayload {
    pub workspace: WorkspacePayload,
}

#[derive(Deserialize)]
pub struct WorkspacePayload {
    pub current_dir: String,
    pub project_dir: String,
}
