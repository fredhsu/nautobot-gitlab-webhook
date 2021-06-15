use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Gitlab {
    host: String,
    project: String,
    branch: String,
    token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Commit {
    id: String,
    branch: String,
    commit_message: String,
    actions: Vec<CommitAction>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommitAction {
    action: String,
    file_path: String,
    content: String,
}
