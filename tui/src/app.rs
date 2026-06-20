use backend::models::Problem;
use backend::rpc::client::RpcClient;

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Loading,
    Ready,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct UserStats {
    pub handle: String,
    pub rating: u32,
    pub solved: u32,
    pub streak: u32,
    pub team: String,
    pub git_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct Tab {
    pub title: String,
}

#[allow(dead_code)]
pub struct App {
    pub state: AppState,
    pub current_tab: usize,
    pub tabs: Vec<Tab>,
    pub problems: Vec<Problem>,
    pub selected_index: usize,
    pub user_stats: UserStats,
    pub rpc_client: Option<RpcClient>,
    pub should_quit: bool,
    pub nvim_path: String,
    pub search_query: String,
    pub show_help: bool,
    pub loading_message: String,
}

impl App {
    pub fn new() -> Self {
        let tabs = vec![
            Tab { title: "Dashboard".into() },
            Tab { title: "Problems".into() },
            Tab { title: "Analytics".into() },
            Tab { title: "Team".into() },
            Tab { title: "Contest".into() },
            Tab { title: "Target".into() },
            Tab { title: "Config".into() },
        ];

        let problems = vec![
            Problem {
                id: "1971D".into(),
                platform: backend::models::Platform::Codeforces,
                title: "Binary Cut".into(),
                rating: Some(1100),
                tags: vec!["dp".into(), "greedy".into(), "strings".into()],
                time_limit: 1000,
                memory_limit: 256,
                statement: String::new(),
                samples: vec![],
                solved: false,
            },
            Problem {
                id: "1971C".into(),
                platform: backend::models::Platform::Codeforces,
                title: "Clock and Strings".into(),
                rating: Some(900),
                tags: vec!["implementation".into(), "strings".into()],
                time_limit: 1000,
                memory_limit: 256,
                statement: String::new(),
                samples: vec![],
                solved: false,
            },
            Problem {
                id: "1900A".into(),
                platform: backend::models::Platform::Codeforces,
                title: "Cover in Water".into(),
                rating: Some(800),
                tags: vec!["constructive algorithms".into(), "implementation".into()],
                time_limit: 1000,
                memory_limit: 256,
                statement: String::new(),
                samples: vec![],
                solved: false,
            },
            Problem {
                id: "1791F".into(),
                platform: backend::models::Platform::Codeforces,
                title: "Range Update Point Query".into(),
                rating: Some(1400),
                tags: vec!["data structures".into(), "greedy".into()],
                time_limit: 1500,
                memory_limit: 256,
                statement: String::new(),
                samples: vec![],
                solved: false,
            },
            Problem {
                id: "1705C".into(),
                platform: backend::models::Platform::Codeforces,
                title: "Mark and His Unfinished Essay".into(),
                rating: Some(1500),
                tags: vec!["implementation".into(), "strings".into()],
                time_limit: 2000,
                memory_limit: 256,
                statement: String::new(),
                samples: vec![],
                solved: false,
            },
        ];

        Self {
            state: AppState::Loading,
            current_tab: 0,
            tabs,
            problems,
            selected_index: 0,
            user_stats: UserStats {
                handle: "alice_cp".into(),
                rating: 1600,
                solved: 145,
                streak: 12,
                team: "None".into(),
                git_enabled: false,
            },
            rpc_client: None,
            should_quit: false,
            nvim_path: "nvim".into(),
            search_query: String::new(),
            show_help: false,
            loading_message: "Loading...".into(),
        }
    }

    pub fn next_tab(&mut self) {
        self.current_tab = (self.current_tab + 1) % self.tabs.len();
    }

    pub fn prev_tab(&mut self) {
        if self.current_tab == 0 {
            self.current_tab = self.tabs.len() - 1;
        } else {
            self.current_tab -= 1;
        }
    }

    pub fn next_item(&mut self) {
        if !self.problems.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.problems.len();
        }
    }

    pub fn prev_item(&mut self) {
        if self.problems.is_empty() {
            return;
        }
        if self.selected_index == 0 {
            self.selected_index = self.problems.len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    pub fn selected_problem(&self) -> Option<&Problem> {
        self.problems.get(self.selected_index)
    }

    pub fn workspace_dir() -> std::path::PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| std::env::current_dir().unwrap())
            .join(".deck")
            .join("workspace")
    }
}
