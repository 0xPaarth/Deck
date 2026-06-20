use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TeamRole {
    Admin,
    Member,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    pub handle: String,
    pub role: TeamRole,
    pub joined_at: DateTime<Utc>,
    pub rating: u32,
    pub solved_today: u32,
    pub streak: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub members: Vec<TeamMember>,
    pub created_at: DateTime<Utc>,
    pub solved_count: HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamStatus {
    pub team: Team,
    pub total_solved: u32,
    pub weekly_solved: u32,
    pub weak_tags: Vec<String>,
    pub last_active: String,
}

pub async fn create_team(name: &str) -> Team {
    let id = format!("team-{}", Utc::now().timestamp_millis());
    Team {
        id,
        name: name.to_string(),
        members: vec![],
        created_at: Utc::now(),
        solved_count: HashMap::new(),
    }
}

pub async fn join_team(team: &mut Team, handle: &str, rating: u32) {
    if !team.members.iter().any(|m| m.handle == handle) {
        team.members.push(TeamMember {
            handle: handle.to_string(),
            role: TeamRole::Member,
            joined_at: Utc::now(),
            rating,
            solved_today: 0,
            streak: 0,
        });
    }
}

pub fn get_team_status(team: &Team) -> TeamStatus {
    let total_solved: u32 = team.solved_count.values().sum();
    let weekly_solved = total_solved.saturating_sub(total_solved / 4); // rough estimate
    let weak_tags = vec!["dp".into(), "graphs".into(), "binary".into()];
    TeamStatus {
        team: team.clone(),
        total_solved,
        weekly_solved,
        weak_tags,
        last_active: "Just now".into(),
    }
}
