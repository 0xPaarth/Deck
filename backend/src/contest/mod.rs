use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contest {
    pub id: String,
    pub platform: String,
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub duration_minutes: u32,
    pub problems: Vec<String>,
    pub is_rated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemStatus {
    pub index: String,
    pub letter: String,
    pub status: String, // "solved", "skipped", "attempting"
    pub score: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContestStatus {
    pub contest: Contest,
    pub time_remaining_seconds: u32,
    pub problems_status: Vec<ProblemStatus>,
    pub score: u32,
    pub rank: u32,
    pub rating_change: i32,
}

pub async fn fetch_upcoming_contests() -> Result<Vec<Contest>, Box<dyn std::error::Error>> {
    // Mock data for now
    let now = Utc::now();
    Ok(vec![
        Contest {
            id: "1234".into(),
            platform: "Codeforces".into(),
            name: "Codeforces Round #1234".into(),
            start_time: now + chrono::Duration::hours(24),
            duration_minutes: 120,
            problems: vec!["A".into(), "B".into(), "C".into(), "D".into(), "E".into()],
            is_rated: true,
        },
        Contest {
            id: "1235".into(),
            platform: "Codeforces".into(),
            name: "Educational Round #99".into(),
            start_time: now + chrono::Duration::hours(72),
            duration_minutes: 120,
            problems: vec!["A".into(), "B".into(), "C".into(), "D".into(), "E".into(), "F".into()],
            is_rated: true,
        },
    ])
}

pub async fn get_contest_status(contest_id: &str) -> Result<ContestStatus, Box<dyn std::error::Error>> {
    let contests = fetch_upcoming_contests().await?;
    let contest = contests.into_iter().find(|c| c.id == contest_id)
        .ok_or("Contest not found")?;

    let now = Utc::now();
    let start = contest.start_time;
    let end = start + chrono::Duration::minutes(contest.duration_minutes as i64);

    let time_remaining = if now < start {
        (start - now).num_seconds().max(0) as u32
    } else if now < end {
        (end - now).num_seconds().max(0) as u32
    } else {
        0
    };

    let problems_status = contest.problems.iter().enumerate().map(|(i, p)| {
        ProblemStatus {
            index: p.clone(),
            letter: p.clone(),
            status: if i == 0 { "solved".into() } else { "skipped".into() },
            score: if i == 0 { 500 } else { 0 },
        }
    }).collect();

    Ok(ContestStatus {
        contest,
        time_remaining_seconds: time_remaining,
        problems_status,
        score: 500,
        rank: 234,
        rating_change: 45,
    })
}
