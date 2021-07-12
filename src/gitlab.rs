use reqwest::header;
use serde::{Deserialize, Serialize};
// pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    SerdeError(serde_json::Error),
    ReqwestError(reqwest::Error),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Gitlab {
    host: String,
    project: u8,
    branch: String,
    token: String,
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
    id: String,
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
    pub fn commits_url(&self) -> String {
        format!(
            "http://{}/api/v4/projects/{}/repository/commits",
            self.host, self.project
        )
    }
    pub fn commit_files(&self, files: Vec<(String, String)>) -> Result<CommitResponse, Error> {
        let mut actions = Vec::new();
        for file in files {
            let (file_path, content) = file;
            actions.push(CommitAction {
                action: "update".to_owned(),
                file_path,
                content,
            });
        }
        self.commit_actions(actions, "updating files".to_owned())
    }
    pub fn commit_actions(
        &self,
        actions: Vec<CommitAction>,
        message: String,
    ) -> Result<CommitResponse, Error> {
        let commit = Commit {
            id: self.project,
            branch: self.branch.to_owned(),
            commit_message: message,
            actions: actions,
        };
        let j = serde_json::to_string(&commit).map_err(Error::SerdeError)?;
        let client = reqwest::blocking::Client::builder()
            .build()
            .map_err(Error::ReqwestError)?;
        let mut res = client
            .post(&self.commits_url())
            .header("PRIVATE-TOKEN", &self.token)
            .json(&commit)
            .send()
            .map_err(Error::ReqwestError)?;
        // let mut buf: Vec<u8> = vec![];
        // res.copy_to(&mut buf).map_err(Error::ReqwestError)?;
        // println!("response: {}", String::from_utf8(buf).unwrap());
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
        let mut files = Vec::new();
        files.push(("test.txt".to_owned(), "test".to_owned()));
        let result = gl.commit_files(files);
        match result {
            Ok(r) => {
                println!("{:?}", r.title);
                assert_eq!(r.author_name, "nautobot-commit".to_owned());
            }
            Err(e) => assert!(false, "Failed commit: {:?}", e),
        }
    }
}
