use crate::analytics::{compute_heatmap, compute_user_stats};
use crate::contest::{fetch_upcoming_contests, get_contest_status};
use crate::fetcher::codeforces::fetch_problem;
use crate::rpc::protocol::{
    platform_from_str, OpenProblemRequest, ProblemDataResponse, RpcRequest, RpcResponse,
    SampleData,
};
use crate::team::{create_team, get_team_status, join_team};
use chrono::Datelike;
use tokio::io::{split, AsyncBufReadExt, AsyncWriteExt, BufReader, ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};

pub const RPC_ADDR: &str = "127.0.0.1:4647";

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(RPC_ADDR).await?;
    println!("[RPC] Server listening on {}", RPC_ADDR);

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: TcpStream) {
    let (reader, mut writer): (ReadHalf<TcpStream>, WriteHalf<TcpStream>) = split(stream);
    let mut lines = BufReader::new(reader).lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let response = match parse_and_handle(&line).await {
            Ok(resp) => resp,
            Err(e) => RpcResponse::err(0, 500, format!("Internal error: {}", e)),
        };

        let json = match serde_json::to_string(&response) {
            Ok(j) => j,
            Err(_) => continue,
        };

        if writer.write_all(json.as_bytes()).await.is_err() {
            break;
        }
        if writer.write_all(b"\n").await.is_err() {
            break;
        }
        let _ = writer.flush().await;
    }
}

async fn parse_and_handle(line: &str) -> Result<RpcResponse, Box<dyn std::error::Error>> {
    let req: RpcRequest = serde_json::from_str(line)?;

    match req.req_type.as_str() {
        "OpenProblem" => handle_open_problem(req).await,
        "GetStats" => handle_get_stats(req).await,
        "GetHeatmap" => handle_get_heatmap(req).await,
        "GetPredictions" => handle_get_predictions(req).await,
        "GetWeakTags" => handle_get_weak_tags(req).await,
        "CreateTeam" => handle_create_team(req).await,
        "JoinTeam" => handle_join_team(req).await,
        "GetTeamStatus" => handle_get_team_status(req).await,
        "ShareSolution" => handle_share_solution(req).await,
        "GetContests" => handle_get_contests(req).await,
        "GetContestStatus" => handle_get_contest_status(req).await,
        "JoinContest" => handle_join_contest(req).await,
        _ => Ok(RpcResponse::err(req.id, 400, "Unknown request type")),
    }
}

async fn handle_open_problem(req: RpcRequest) -> Result<RpcResponse, Box<dyn std::error::Error>> {
    let payload: OpenProblemRequest = serde_json::from_value(req.payload)?;
    let _platform = platform_from_str(&payload.platform)
        .ok_or_else(|| format!("Unknown platform: {}", payload.platform))?;

    let problem = fetch_problem(&payload.problem_id).await?;

    let resp = ProblemDataResponse {
        title: problem.title.clone(),
        statement: problem.statement,
        samples: problem
            .samples
            .into_iter()
            .map(|s| SampleData {
                input: s.input,
                output: s.output,
            })
            .collect(),
        rating: problem.rating,
        tags: problem.tags,
    };

    Ok(RpcResponse::ok(req.id, "ProblemData", resp))
}

async fn handle_get_stats(req: RpcRequest) -> Result<RpcResponse, Box<dyn std::error::Error>> {
    let handle = req
        .payload
        .get("handle")
        .and_then(|v| v.as_str())
        .unwrap_or("alice_cp");
    let stats = compute_user_stats(handle)?;
    Ok(RpcResponse::ok(req.id, "UserStats", stats))
}

async fn handle_get_heatmap(
    req: RpcRequest,
) -> Result<RpcResponse, Box<dyn std::error::Error>> {
    let handle = req
        .payload
        .get("handle")
        .and_then(|v| v.as_str())
        .unwrap_or("alice_cp");
    let year = req
        .payload
        .get("year")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32)
        .unwrap_or_else(|| chrono::Utc::now().year());
    let heatmap = compute_heatmap(handle, year)?;
    Ok(RpcResponse::ok(req.id, "HeatmapData", heatmap))
}

async fn handle_get_predictions(
    req: RpcRequest,
) -> Result<RpcResponse, Box<dyn std::error::Error>> {
    let handle = req
        .payload
        .get("handle")
        .and_then(|v| v.as_str())
        .unwrap_or("alice_cp");
    let stats = compute_user_stats(handle)?;
    let predictions = serde_json::json!({
        "predicted_rating": stats.predicted_rating,
        "time_to_next_milestone": stats.time_to_next_milestone,
        "recommendations": stats.weak_tags.iter().take(3).map(|w| &w.recommendation).collect::<Vec<_>>(),
    });
    Ok(RpcResponse::ok(req.id, "Predictions", predictions))
}

async fn handle_get_weak_tags(
    req: RpcRequest,
) -> Result<RpcResponse, Box<dyn std::error::Error>> {
    let handle = req
        .payload
        .get("handle")
        .and_then(|v| v.as_str())
        .unwrap_or("alice_cp");
    let stats = compute_user_stats(handle)?;
    Ok(RpcResponse::ok(req.id, "WeakTags", stats.weak_tags))
}

async fn handle_create_team(
    req: RpcRequest,
) -> Result<RpcResponse, Box<dyn std::error::Error>> {
    let name = req
        .payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("New Team");
    let team = create_team(name).await;
    Ok(RpcResponse::ok(req.id, "TeamCreated", team))
}

async fn handle_join_team(
    req: RpcRequest,
) -> Result<RpcResponse, Box<dyn std::error::Error>> {
    let team_id = req.payload.get("team_id").and_then(|v| v.as_str()).unwrap_or("");
    let handle = req.payload.get("handle").and_then(|v| v.as_str()).unwrap_or("");
    let rating = req.payload.get("rating").and_then(|v| v.as_u64()).unwrap_or(1500) as u32;
    // Simplified: return mock team
    let mut team = create_team(team_id).await;
    join_team(&mut team, handle, rating).await;
    let status = get_team_status(&team);
    Ok(RpcResponse::ok(req.id, "TeamStatus", status))
}

async fn handle_get_team_status(
    req: RpcRequest,
) -> Result<RpcResponse, Box<dyn std::error::Error>> {
    let team_id = req.payload.get("team_id").and_then(|v| v.as_str()).unwrap_or("");
    let name = if team_id.is_empty() { "CP-Squad" } else { team_id };
    let mut team = create_team(name).await;
    join_team(&mut team, "alice_cp", 1600).await;
    join_team(&mut team, "bob", 1400).await;
    join_team(&mut team, "charlie", 1700).await;
    team.solved_count.insert("alice_cp".into(), 145);
    team.solved_count.insert("bob".into(), 89);
    team.solved_count.insert("charlie".into(), 210);
    let status = get_team_status(&team);
    Ok(RpcResponse::ok(req.id, "TeamStatus", status))
}

async fn handle_share_solution(
    _req: RpcRequest,
) -> Result<RpcResponse, Box<dyn std::error::Error>> {
    Ok(RpcResponse::ok(_req.id, "Shared", serde_json::json!({ "shared": true })))
}

async fn handle_get_contests(
    req: RpcRequest,
) -> Result<RpcResponse, Box<dyn std::error::Error>> {
    let contests = fetch_upcoming_contests().await?;
    Ok(RpcResponse::ok(req.id, "Contests", contests))
}

async fn handle_get_contest_status(
    req: RpcRequest,
) -> Result<RpcResponse, Box<dyn std::error::Error>> {
    let contest_id = req
        .payload
        .get("contest_id")
        .and_then(|v| v.as_str())
        .unwrap_or("1234");
    let status = get_contest_status(contest_id).await?;
    Ok(RpcResponse::ok(req.id, "ContestStatus", status))
}

async fn handle_join_contest(
    req: RpcRequest,
) -> Result<RpcResponse, Box<dyn std::error::Error>> {
    let contest_id = req
        .payload
        .get("contest_id")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    Ok(RpcResponse::ok(
        req.id,
        "JoinedContest",
        serde_json::json!({ "contest_id": contest_id }),
    ))
}
