use backend::db::{connect, upsert_user};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "deck",
    about = "Deck — terminal-first competitive programming platform",
    version = env!("CARGO_PKG_VERSION"),
    long_about = "Deck is a terminal-first competitive programming platform with Neovim integration,\nGit sync, team collaboration, and deep analytics.\n\nCommands:\n  deck init           Initialize configuration and workspace\n  deck fetch <id>     Download and display a Codeforces problem\n  deck tui            Start the interactive TUI (same as deck-tui)\n  deck --version      Show version"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize Deck configuration and workspace
    Init,
    /// Fetch a problem from a platform
    Fetch {
        /// Problem ID, e.g. 1971D
        problem_id: String,
    },
    /// Start the interactive TUI
    Tui,
    /// Show user statistics
    Stats {
        /// User handle
        #[arg(default_value = "alice_cp")]
        handle: String,
    },
    /// Open a problem in Neovim
    Open {
        /// Problem ID, e.g. 1971D
        problem_id: String,
    },
    /// Configure Deck settings
    Config,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => init_deck().await,
        Some(Commands::Fetch { problem_id }) => fetch_and_print(&problem_id).await,
        Some(Commands::Tui) => start_tui().await,
        Some(Commands::Stats { handle }) => show_stats(&handle).await,
        Some(Commands::Open { problem_id }) => open_in_neovim(&problem_id).await,
        Some(Commands::Config) => show_config().await,
        None => {
            eprintln!("Usage: deck [COMMAND]\n");
            eprintln!("Commands:");
            eprintln!("  init    Initialize configuration and workspace");
            eprintln!("  fetch   Download a Codeforces problem");
            eprintln!("  tui     Start the interactive TUI");
            eprintln!("  stats   Show user statistics");
            eprintln!("  open    Open a problem in Neovim");
            eprintln!("  config  Show current configuration");
            eprintln!();
            eprintln!("Run 'deck --help' for more information.");
            std::process::exit(1);
        }
    }
}

async fn init_deck() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Deck v{}", env!("CARGO_PKG_VERSION"));
    println!("Initializing Deck...\n");

    // Create directories
    let home = dirs::home_dir().unwrap_or_else(|| std::env::current_dir().unwrap());
    let deck_dir = home.join(".deck");
    let workspace = deck_dir.join("workspace");
    let bin = deck_dir.join("bin");

    for dir in [&deck_dir, &workspace, &bin] {
        tokio::fs::create_dir_all(dir).await?;
        println!("  ✅ Created {}", dir.display());
    }

    // Initialize database
    let conn = connect()?;
    println!("  ✅ Initialized SQLite database");

    // Create default user
    upsert_user(&conn, "alice_cp", 1500, 0, 0, 1500)?;
    println!("  ✅ Created default user 'alice_cp'");

    // Create config file
    let config_path = deck_dir.join("config.toml");
    if !config_path.exists() {
        let config = r#"# Deck Configuration
[general]
handle = "alice_cp"
default_language = "cpp"
rating_goal = 1700

[git]
enabled = true
repo_path = "~/.deck/repo"
auto_commit = true
auto_push = false

[tui]
theme = "default"
"#;
        tokio::fs::write(&config_path, config).await?;
        println!("  ✅ Created config.toml");
    }

    println!("\n✨ Deck is ready! Run 'deck tui' to start.");
    Ok(())
}

async fn fetch_and_print(problem_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let problem = backend::fetcher::codeforces::fetch_problem(problem_id).await?;
    println!("Fetched problem: {} - {}", problem.id, problem.title);
    println!("Platform: {:?}", problem.platform);
    println!("Time limit: {} ms", problem.time_limit);
    println!("Memory limit: {} MB", problem.memory_limit);
    println!("Tags: {:?}", problem.tags);
    println!("Samples: {} test case(s)", problem.samples.len());
    for (i, sample) in problem.samples.iter().enumerate() {
        println!("\n--- Sample {} ---", i + 1);
        println!("Input:\n{}", sample.input);
        println!("Output:\n{}", sample.output);
    }
    println!("\n--- Statement (first 500 chars) ---");
    println!(
        "{}",
        &problem.statement[..problem.statement.len().min(500)]
    );
    Ok(())
}

async fn start_tui() -> Result<(), Box<dyn std::error::Error>> {
    // The TUI is in the tui crate — this is just a convenience wrapper
    eprintln!("Starting TUI... use 'cargo run --bin deck-tui' or run the deck-tui binary directly.");
    std::process::exit(1);
}

async fn show_stats(handle: &str) -> Result<(), Box<dyn std::error::Error>> {
    let stats = backend::analytics::compute_user_stats(handle)?;
    println!("📊 Stats for {}\n", stats.handle);
    println!("  Rating:        {}", stats.rating);
    println!("  Solved:        {}", stats.solved);
    println!("  Streak:        {} days", stats.streak);
    println!("  Max Rating:    {}", stats.max_rating);
    println!("  Avg Time:      {} mins", stats.avg_solve_time / 60);
    println!("  Predicted:     {}", stats.predicted_rating);
    println!("  Next Milestone: ~{} days", stats.time_to_next_milestone);
    println!("\n  Weak tags:");
    for wt in &stats.weak_tags[..stats.weak_tags.len().min(5)] {
        let icon = if wt.priority >= 4 { "❌" } else { "⚠️" };
        println!(
            "    {} {}  ({}%, {})",
            icon, wt.name, wt.accuracy as u32, wt.recommendation
        );
    }
    Ok(())
}

async fn open_in_neovim(problem_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let problem = backend::fetcher::codeforces::fetch_problem(problem_id).await?;
    let workspace = dirs::home_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap())
        .join(".deck")
        .join("workspace");
    tokio::fs::create_dir_all(&workspace).await?;

    let safe_title = problem
        .title
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != '_', "_")
        .replace("__", "_");
    let file_path = workspace.join(format!("{}_{}.cpp", problem_id, safe_title));

    let template = format!(
        "// {} - {}\n\
         #include <bits/stdc++.h>\n\
         using namespace std;\n\
         \n\
         int main() {{\n\
             ios::sync_with_stdio(false);\n\
             cin.tie(nullptr);\n\
             \n\
             return 0;\n\
         }}\n",
        problem_id, problem.title
    );
    tokio::fs::write(&file_path, template).await?;

    println!("Opening {} in Neovim...", file_path.display());
    std::process::Command::new("nvim")
        .arg(&file_path)
        .status()?;

    Ok(())
}

async fn show_config() -> Result<(), Box<dyn std::error::Error>> {
    let home = dirs::home_dir().unwrap_or_else(|| std::env::current_dir().unwrap());
    let config_path = home.join(".deck").join("config.toml");
    if config_path.exists() {
        let content = tokio::fs::read_to_string(&config_path).await?;
        println!("{}", content);
    } else {
        println!("No config found. Run 'deck init' first.");
    }
    Ok(())
}
