use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub id: String,
    pub platform: Platform,
    pub title: String,
    pub rating: Option<u32>,
    pub tags: Vec<String>,
    pub time_limit: u32,   // ms
    pub memory_limit: u32, // MB
    pub statement: String, // Markdown
    pub samples: Vec<TestCase>,
    pub solved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Platform {
    Codeforces,
    CSES,
    AtCoder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solution {
    pub problem_id: String,
    pub file_path: PathBuf,
    pub language: Language,
    pub verdict: Verdict,
    pub execution_time: Option<u32>,
    pub submitted_at: DateTime<Utc>,
    pub commit_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    Cpp,
    Python,
    Rust,
    Java,
    Go,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Verdict {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    RuntimeError,
    CompilationError,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub handle: String,
    pub rating: u32,
    pub solved: u32,
    pub streak: u32,
    pub teams: Vec<String>,
    pub git_config: Option<GitConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    pub repo_path: PathBuf,
    pub auto_commit: bool,
    pub auto_push: bool,
}
