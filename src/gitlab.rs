// use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Debug)]
pub enum Error {
    SerdeError(serde_json::Error),
    ReqwestError(reqwest::Error),
}

#[derive(Clone, Debug)]
pub struct FileEntry {
    pub file_path: String,
    pub content: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Gitlab {
    pub host: String,
    pub project: u8,
    pub branch: String,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Commit {
    id: u8,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct CommitResponse {
    pub id: String,
    short_id: String,
    title: String,
    author_name: String,
    author_email: String,
    committer_name: String,
    committer_email: String,
    created_at: String,
    message: String,
    parent_ids: Vec<String>,
    project_id: u8,
    committed_date: String,
    authored_date: String,
    stats: Stats,
    status: Option<String>,
    web_url: String,
    last_pipeline: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stats {
    additions: u8,
    deletions: u8,
    total: u8,
}
impl Gitlab {
    // TODO: Add a new() constructor - check for branch/project on construction
    pub fn commits_url(&self) -> String {
        format!(
            "http://{}/api/v4/projects/{}/repository/commits",
            self.host, self.project
        )
    }
    pub fn commit_files(&self, files: Vec<FileEntry>) -> Result<CommitResponse, Error> {
        let mut actions = Vec::new();
        for file in files {
            actions.push(CommitAction {
                action: "update".to_owned(),
                file_path: file.file_path,
                content: file.content,
            });
        }
        self.commit_actions(actions, "updating files".to_owned())
    }
    pub fn commit_actions(
        &self,
        actions: Vec<CommitAction>,
        commit_message: String,
    ) -> Result<CommitResponse, Error> {
        // TODO: Check for branch
        let commit = Commit {
            id: self.project,
            branch: self.branch.to_owned(),
            commit_message,
            actions,
        };
        let client = reqwest::blocking::Client::builder()
            .build()
            .map_err(Error::ReqwestError)?;
        let res = client
            .post(&self.commits_url())
            .header("PRIVATE-TOKEN", &self.token)
            .json(&commit)
            .send()
            .map_err(Error::ReqwestError)?;
        //TODO: Add some error handling for 4xx responses
        let cr: CommitResponse = res.json().map_err(Error::ReqwestError)?;
        Ok(cr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_commit_files() {
        let gl = Gitlab {
            host: "dmz-gitlab.sjc.aristanetworks.com".to_owned(),
            project: 5,
            branch: "nautobot".to_owned(),
            token: "NnnPwyihFTVRsnqk_dfi".to_owned(),
        };
        let files = vec![FileEntry {
            file_path: "test.txt".to_owned(),
            content: "test".to_owned(),
        }];
        let result = gl.commit_files(files).unwrap();
        assert_eq!(result.author_name, "nautobot-commit".to_owned());
    }
}
