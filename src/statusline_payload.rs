use serde::Deserialize;

#[derive(Deserialize)]
pub struct StatuslinePayload {
    pub model: ModelPayload,
    pub workspace: WorkspacePayload,
}

#[derive(Deserialize)]
pub struct ModelPayload {
    pub id: String,
}

#[derive(Deserialize)]
pub struct WorkspacePayload {
    pub current_dir: String,
    pub project_dir: String,
}
