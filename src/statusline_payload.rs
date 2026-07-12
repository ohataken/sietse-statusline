use serde::Deserialize;

#[derive(Deserialize)]
pub struct StatuslinePayload {
    pub model: ModelPayload,
    pub workspace: WorkspacePayload,
    pub context_window: ContextWindowPayload,
}

#[derive(Deserialize)]
pub struct ContextWindowPayload {
    pub total_input_tokens: u64,
}

#[derive(Deserialize)]
pub struct ModelPayload {
    pub id: String,
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct WorkspacePayload {
    pub current_dir: String,
    pub project_dir: String,
}
