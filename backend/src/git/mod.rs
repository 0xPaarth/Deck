use crate::models::{GitConfig, Solution};
use git2::{Error as GitError, Repository, Signature};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub struct GitManager {
    repo: Repository,
    config: GitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolutionMetadata {
    pub problem_id: String,
    pub platform: String,
    pub title: String,
    pub rating: Option<u32>,
    pub tags: Vec<String>,
    pub solved_at: String,
    pub time_taken_seconds: Option<u32>,
    pub language: String,
    pub execution_time_ms: Option<u32>,
    pub memory_used_kb: Option<u32>,
    pub approach: String,
    pub complexity: String,
    pub attempts: u32,
}

impl GitManager {
    pub fn new(workspace_path: &Path, config: GitConfig) -> Result<Self, GitError> {
        let repo = if workspace_path.join(".git").exists() {
            Repository::open(workspace_path)?
        } else {
            Repository::init(workspace_path)?
        };
        Ok(Self { repo, config })
    }

    pub fn init_repo(&self) -> Result<(), GitError> {
        // Ensure gitignore
        let gitignore = self.config.repo_path.join(".gitignore");
        if !gitignore.exists() {
            let _ = fs::write(&gitignore, "*.out\n*.exe\n*.class\n__pycache__/\n");
        }
        Ok(())
    }

    pub fn commit_solution(&self, solution: &Solution, metadata: &SolutionMetadata) -> Result<String, Box<dyn std::error::Error>> {
        let platform_dir = match metadata.platform.to_lowercase().as_str() {
            "codeforces" => "codeforces",
            "cses" => "cses",
            "atcoder" => "atcoder",
            _ => "other",
        };

        let problem_dir = self.config.repo_path
            .join("solutions")
            .join(platform_dir)
            .join(&metadata.problem_id);

        fs::create_dir_all(&problem_dir)?;

        // Copy solution file
        let ext = match metadata.language.as_str() {
            "rust" => "rs",
            "python" => "py",
            "go" => "go",
            "java" => "java",
            _ => "cpp",
        };
        let dest = problem_dir.join(format!("solution.{}", ext));
        fs::copy(&solution.file_path, &dest)?;

        // Write metadata.json
        let meta_path = problem_dir.join("metadata.json");
        let meta_json = serde_json::to_string_pretty(metadata)?;
        fs::write(&meta_path, meta_json)?;

        // Git add
        let mut index = self.repo.index()?;
        index.add_path(&self.path_relative_to_repo(&dest)?)?;
        index.add_path(&self.path_relative_to_repo(&meta_path)?)?;
        index.write()?;

        // Commit
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;
        let sig = self.signature()?;
        let message = format!(
            "Solved {} {} ({}) [{}] {}",
            metadata.platform.to_uppercase(),
            metadata.problem_id,
            metadata.title,
            metadata.rating.map(|r| r.to_string()).unwrap_or_else(|| "?".into()),
            metadata.tags.join(", "),
        );

        let parent = match self.repo.head() {
            Ok(head) => {
                let commit = head.peel_to_commit()?;
                vec![commit]
            }
            Err(_) => vec![],
        };

        let parents_refs: Vec<&git2::Commit> = parent.iter().collect();
        let commit_id = self.repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &message,
            &tree,
            &parents_refs,
        )?;

        // Update index for next write
        index.write()?;

        let hash = commit_id.to_string();

        // Regenerate README
        let _ = self.generate_readme();

        // Auto-push if enabled
        if self.config.auto_push {
            let _ = self.push();
        }

        Ok(hash)
    }

    pub fn push(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(mut remote) = self.repo.find_remote("origin") {
            let mut callbacks = git2::RemoteCallbacks::new();
            callbacks.credentials(|_url, username_from_url, _allowed_types| {
                git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
            });
            let mut push_options = git2::PushOptions::new();
            push_options.remote_callbacks(callbacks);
            let _branch = self
                .config
                .repo_path
                .join(".git/HEAD")
                .to_str()
                .unwrap_or("main")
                .to_string();
            let refspec = format!("refs/heads/main:refs/heads/main");
            remote.push(&[&refspec], Some(&mut push_options))?;
        }
        Ok(())
    }

    pub fn generate_readme(&self) -> Result<(), Box<dyn std::error::Error>> {
        let readme_path = self.config.repo_path.join("README.md");
        let solutions_dir = self.config.repo_path.join("solutions");

        let mut content = String::from("# 🏆 My CP Solutions\n\n");
        content.push_str("## Recent Solutions\n\n");
        content.push_str("| Problem | Platform | Rating | Tags | Date |\n");
        content.push_str("|---------|----------|--------|------|------|\n");

        if solutions_dir.exists() {
            for entry in walkdir::WalkDir::new(&solutions_dir)
                .max_depth(3)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_name() == "metadata.json" {
                    if let Ok(json) = fs::read_to_string(entry.path()) {
                        if let Ok(meta) = serde_json::from_str::<SolutionMetadata>(&json) {
                            let date = meta.solved_at.split('T').next().unwrap_or("");
                            content.push_str(&format!(
                                "| {} | {} | {} | {} | {} |\n",
                                meta.problem_id,
                                meta.platform,
                                meta.rating.map(|r| r.to_string()).unwrap_or_else(|| "?".into()),
                                meta.tags.join(", "),
                                date
                            ));
                        }
                    }
                }
            }
        }

        content.push_str("\n_Generated by Deck_\n");
        fs::write(&readme_path, content)?;

        let mut index = self.repo.index()?;
        index.add_path(Path::new("README.md"))?;
        index.write()?;

        Ok(())
    }

    fn path_relative_to_repo(&self, abs: &Path) -> Result<PathBuf, GitError> {
        let repo_path = self.repo.workdir().unwrap_or(self.config.repo_path.as_ref());
        Ok(abs.strip_prefix(repo_path).unwrap_or(abs).to_path_buf())
    }

    fn signature(&self) -> Result<Signature<'_>, GitError> {
        Signature::now("Deck CP", "deck@localhost")
    }
}
