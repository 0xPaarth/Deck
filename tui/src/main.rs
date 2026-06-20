mod app;
mod tabs;
mod ui;
mod widgets;

use app::{App, AppState};
use backend::rpc::server::{run_server, socket_path};
use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use futures::StreamExt;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{stdout, Stdout};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start RPC server in background
    tokio::spawn(async {
        if let Err(e) = run_server().await {
            eprintln!("[RPC] Server error: {}", e);
        }
    });

    // Give server time to bind
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    let mut app = App::new();

    // Connect RPC client
    match backend::rpc::client::RpcClient::connect(socket_path()).await {
        Ok(client) => {
            app.rpc_client = Some(client);
            app.state = AppState::Ready;
        }
        Err(e) => {
            app.state = AppState::Error(format!("RPC connect failed: {}", e));
        }
    }

    let mut terminal = setup_terminal()?;
    let result = run_app(&mut terminal, &mut app).await;
    restore_terminal(&mut terminal)?;

    if let Err(ref e) = result {
        eprintln!("TUI error: {}", e);
    }

    result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn std::error::Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.hide_cursor()?;
    terminal.clear()?;
    Ok(terminal)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.show_cursor()?;
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = EventStream::new();
    let mut tick = tokio::time::interval(tokio::time::Duration::from_millis(250));

    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        let event = tokio::select! {
            _ = tick.tick() => None,
            maybe_event = reader.next() => {
                match maybe_event {
                    Some(Ok(Event::Key(key))) => Some(key),
                    Some(Ok(_)) => None,
                    Some(Err(e)) => {
                        app.state = AppState::Error(format!("Input error: {}", e));
                        None
                    }
                    None => {
                        break;
                    }
                }
            }
        };

        if let Some(key) = event {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => {
                    app.should_quit = true;
                }
                KeyCode::Char('?') => {
                    app.show_help = !app.show_help;
                }
                KeyCode::Tab => {
                    if key.modifiers.contains(KeyModifiers::SHIFT) {
                        app.prev_tab();
                    } else {
                        app.next_tab();
                    }
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if app.current_tab == 1 {
                        app.next_item();
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if app.current_tab == 1 {
                        app.prev_item();
                    }
                }
                KeyCode::Char('n') | KeyCode::Enter => {
                    if let Some(problem) = app.selected_problem().cloned() {
                        open_problem(app, &problem.id, terminal).await?;
                    }
                }
                KeyCode::Char('r') => {
                    // Refresh placeholder
                }
                _ => {}
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

async fn open_problem(
    app: &mut App,
    problem_id: &str,
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Send OpenProblem RPC and get title
    let title = if let Some(client) = app.rpc_client.as_mut() {
        let payload = serde_json::json!({
            "problem_id": problem_id,
            "platform": "codeforces"
        });
        match client.request("OpenProblem", payload).await {
            Ok(resp) => {
                if let Some(err) = resp.error {
                    app.state = AppState::Error(format!("RPC {}: {}", err.code, err.message));
                    return Ok(());
                }
                let data: backend::rpc::protocol::ProblemDataResponse =
                    serde_json::from_value(resp.payload).unwrap_or_else(|_| {
                        backend::rpc::protocol::ProblemDataResponse {
                            title: problem_id.to_string(),
                            statement: String::new(),
                            samples: vec![],
                            rating: None,
                            tags: vec![],
                        }
                    });
                data.title
            }
            Err(e) => {
                app.state = AppState::Error(format!("RPC request failed: {}", e));
                return Ok(());
            }
        }
    } else {
        problem_id.to_string()
    };

    // Create workspace file with template
    let workspace = App::workspace_dir();
    tokio::fs::create_dir_all(&workspace).await?;
    let safe_title = title
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
        problem_id, title
    );
    tokio::fs::write(&file_path, template).await?;

    // Suspend TUI and launch Neovim
    terminal.show_cursor()?;
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    let status = tokio::process::Command::new(&app.nvim_path)
        .arg(&file_path)
        .status()
        .await;

    if let Err(e) = status {
        app.state = AppState::Error(format!("Failed to launch nvim: {}", e));
    }

    // Resume TUI
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    Ok(())
}
