use crate::db::{connect, get_all_solutions, get_user};
use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicStats {
    pub solved: u32,
    pub attempted: u32,
    pub accuracy: f32,
    pub avg_time_seconds: u32,
    pub last_solved: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeakTag {
    pub name: String,
    pub accuracy: f32,
    pub priority: u8,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeBucket {
    pub label: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub handle: String,
    pub rating: u32,
    pub solved: u32,
    pub streak: u32,
    pub max_rating: u32,
    pub topic_accuracy: HashMap<String, TopicStats>,
    pub avg_solve_time: u32,
    pub solve_time_distribution: Vec<TimeBucket>,
    pub weak_tags: Vec<WeakTag>,
    pub predicted_rating: u32,
    pub time_to_next_milestone: u32,
    pub recent_problems: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatmapData {
    pub year: i32,
    pub days: HashMap<String, u32>, // "YYYY-MM-DD" -> count
}

pub fn compute_user_stats(handle: &str) -> Result<UserStats, Box<dyn std::error::Error>> {
    let conn = connect()?;
    let user = get_user(&conn, handle)?;
    let solutions = get_all_solutions(&conn)?;

    let mut topic_map: HashMap<String, (u32, u32, u32, Option<chrono::DateTime<Utc>>)> =
        HashMap::new();
    let mut time_buckets = vec![
        ("0-15m", 0u32),
        ("15-30m", 0u32),
        ("30-60m", 0u32),
        ("60m+", 0u32),
    ];
    let mut total_time = 0u64;
    let mut recent_problems = Vec::new();

    for sol in &solutions[..solutions.len().min(50)] {
        for tag in &sol.tags {
            let entry = topic_map.entry(tag.clone()).or_insert((0, 0, 0, None));
            entry.0 += 1;
            entry.1 += sol.attempts;
            if let Some(t) = sol.time_taken_seconds {
                entry.2 += t;
            }
            if entry.3.is_none() || sol.solved_at > entry.3.unwrap() {
                entry.3 = Some(sol.solved_at);
            }
        }

        if let Some(t) = sol.time_taken_seconds {
            total_time += t as u64;
            if t <= 900 {
                time_buckets[0].1 += 1;
            } else if t <= 1800 {
                time_buckets[1].1 += 1;
            } else if t <= 3600 {
                time_buckets[2].1 += 1;
            } else {
                time_buckets[3].1 += 1;
            }
        }

        if recent_problems.len() < 5 {
            recent_problems.push(format!("{} - {}", sol.problem_id, sol.platform));
        }
    }

    let solved_count = solutions.len() as u32;
    let avg_time = if solved_count > 0 {
        (total_time / solved_count as u64) as u32
    } else {
        0
    };

    let mut topic_accuracy = HashMap::new();
    let mut weak_tags = Vec::new();

    for (tag, (solved, attempted, total_tag_time, last)) in topic_map {
        let accuracy = if attempted > 0 {
            (solved as f32 / attempted as f32) * 100.0
        } else {
            100.0
        };
        let avg_tag_time = if solved > 0 { total_tag_time / solved } else { 0 };

        topic_accuracy.insert(
            tag.clone(),
            TopicStats {
                solved,
                attempted,
                accuracy,
                avg_time_seconds: avg_tag_time,
                last_solved: last.map(|d| d.format("%Y-%m-%d").to_string()),
            },
        );

        if accuracy < 75.0 {
            let priority = if accuracy < 40.0 {
                5
            } else if accuracy < 55.0 {
                4
            } else {
                3
            };
            weak_tags.push(WeakTag {
                name: tag.clone(),
                accuracy,
                priority,
                recommendation: format!(
                    "Practice {}-{} {} problems",
                    if solved > 0 { 800 + solved * 100 } else { 800 },
                    if solved > 0 { 1000 + solved * 100 } else { 1200 },
                    tag
                ),
            });
        }
    }

    weak_tags.sort_by(|a, b| b.priority.cmp(&a.priority));
    weak_tags.truncate(10);

    let rating = user.as_ref().map(|u| u.rating).unwrap_or(1500);
    let solved = user.as_ref().map(|u| u.solved).unwrap_or(solved_count);
    let streak = user.as_ref().map(|u| u.streak).unwrap_or(0);
    let max_rating = user.as_ref().map(|u| u.max_rating).unwrap_or(rating);

    let solved_for_pred = solved.max(1);
    let growth = (solved_for_pred / 10).min(20) * 50;
    let predicted_rating = (rating + growth).min(3500);

    let next_milestone = ((rating / 100 + 1) * 100).saturating_sub(rating);
    let time_to_next = if next_milestone > 0 { (next_milestone / 10).max(1) } else { 0 };

    Ok(UserStats {
        handle: handle.to_string(),
        rating,
        solved,
        streak,
        max_rating,
        topic_accuracy,
        avg_solve_time: avg_time,
        solve_time_distribution: time_buckets
            .into_iter()
            .map(|(label, count)| TimeBucket {
                label: label.to_string(),
                count,
            })
            .collect(),
        weak_tags,
        predicted_rating,
        time_to_next_milestone: time_to_next,
        recent_problems,
    })
}

pub fn compute_heatmap(
    _handle: &str,
    year: i32,
) -> Result<HeatmapData, Box<dyn std::error::Error>> {
    let conn = connect()?;
    let solutions = get_all_solutions(&conn)?;

    let mut days: HashMap<String, u32> = HashMap::new();
    for sol in &solutions {
        if sol.solved_at.year() == year {
            let date_key = sol.solved_at.format("%Y-%m-%d").to_string();
            *days.entry(date_key).or_insert(0) += 1;
        }
    }

    Ok(HeatmapData { year, days })
}
