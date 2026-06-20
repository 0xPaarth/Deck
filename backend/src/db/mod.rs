use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result as SqlResult};
use serde_json;
use std::path::PathBuf;

pub fn db_path() -> PathBuf {
    dirs::cache_dir()
        .unwrap_or_else(|| std::env::temp_dir())
        .join("deck")
        .join("deck.db")
}

fn ensure_dir(path: &PathBuf) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
}

pub fn connect() -> SqlResult<Connection> {
    let path = db_path();
    ensure_dir(&path);
    let conn = Connection::open(&path)?;
    run_migrations(&conn)?;
    Ok(conn)
}

fn run_migrations(conn: &Connection) -> SqlResult<()> {
    conn.execute_batch(
        "PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS users (
            handle TEXT PRIMARY KEY,
            rating INTEGER NOT NULL DEFAULT 0,
            solved INTEGER NOT NULL DEFAULT 0,
            streak INTEGER NOT NULL DEFAULT 0,
            max_rating INTEGER NOT NULL DEFAULT 0,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS problems (
            id TEXT PRIMARY KEY,
            platform TEXT NOT NULL,
            title TEXT NOT NULL,
            rating INTEGER,
            tags TEXT, -- JSON array
            statement TEXT,
            samples TEXT -- JSON array
        );

        CREATE TABLE IF NOT EXISTS solutions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            problem_id TEXT NOT NULL,
            platform TEXT NOT NULL,
            solved_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            time_taken_seconds INTEGER,
            execution_time_ms INTEGER,
            memory_used_kb INTEGER,
            attempts INTEGER NOT NULL DEFAULT 1,
            rating INTEGER,
            tags TEXT, -- JSON array
            language TEXT,
            commit_hash TEXT,
            FOREIGN KEY (problem_id) REFERENCES problems(id)
        );

        CREATE TABLE IF NOT EXISTS teams (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            members TEXT, -- JSON array
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE INDEX IF NOT EXISTS idx_solutions_problem_id ON solutions(problem_id);
        CREATE INDEX IF NOT EXISTS idx_solutions_solved_at ON solutions(solved_at);
        CREATE INDEX IF NOT EXISTS idx_solutions_platform ON solutions(platform);
        "
    )?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct UserRow {
    pub handle: String,
    pub rating: u32,
    pub solved: u32,
    pub streak: u32,
    pub max_rating: u32,
    pub updated_at: DateTime<Utc>,
}

pub fn upsert_user(
    conn: &Connection,
    handle: &str,
    rating: u32,
    solved: u32,
    streak: u32,
    max_rating: u32,
) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO users (handle, rating, solved, streak, max_rating, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(handle) DO UPDATE SET
             rating=excluded.rating,
             solved=excluded.solved,
             streak=excluded.streak,
             max_rating=excluded.max_rating,
             updated_at=excluded.updated_at",
        params![
            handle,
            rating,
            solved,
            streak,
            max_rating,
            Utc::now().to_rfc3339(),
        ],
    )?;
    Ok(())
}

pub fn get_user(conn: &Connection, handle: &str) -> SqlResult<Option<UserRow>> {
    let mut stmt = conn.prepare(
        "SELECT handle, rating, solved, streak, max_rating, updated_at FROM users WHERE handle = ?1"
    )?;
    let mut rows = stmt.query_map([handle], |row| {
        let updated_at_str: String = row.get(5)?;
        let updated_at = updated_at_str
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        Ok(UserRow {
            handle: row.get(0)?,
            rating: row.get(1)?,
            solved: row.get(2)?,
            streak: row.get(3)?,
            max_rating: row.get(4)?,
            updated_at,
        })
    })?;
    rows.next().transpose()
}

#[derive(Debug, Clone)]
pub struct ProblemRow {
    pub id: String,
    pub platform: String,
    pub title: String,
    pub rating: Option<u32>,
    pub tags: Vec<String>,
    pub statement: String,
    pub samples: Vec<crate::models::TestCase>,
}

pub fn upsert_problem(conn: &Connection, problem: &crate::models::Problem) -> SqlResult<()> {
    let tags_json = serde_json::to_string(&problem.tags).unwrap_or_else(|_| "[]".to_string());
    let samples_json = serde_json::to_string(&problem.samples).unwrap_or_else(|_| "[]".to_string());
    conn.execute(
        "INSERT INTO problems (id, platform, title, rating, tags, statement, samples)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
         ON CONFLICT(id) DO UPDATE SET
             title=excluded.title,
             rating=excluded.rating,
             tags=excluded.tags,
             statement=excluded.statement,
             samples=excluded.samples",
        params![
            problem.id,
            format!("{:?}", problem.platform),
            problem.title,
            problem.rating,
            tags_json,
            problem.statement,
            samples_json,
        ],
    )?;
    Ok(())
}

pub fn get_problem(conn: &Connection, id: &str) -> SqlResult<Option<ProblemRow>> {
    let mut stmt = conn.prepare(
        "SELECT id, platform, title, rating, tags, statement, samples FROM problems WHERE id = ?1"
    )?;
    let mut rows = stmt.query_map([id], |row| {
        let tags: Vec<String> = serde_json::from_str(&row.get::<_, String>(4)?).unwrap_or_default();
        let samples: Vec<crate::models::TestCase> =
            serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default();
        Ok(ProblemRow {
            id: row.get(0)?,
            platform: row.get(1)?,
            title: row.get(2)?,
            rating: row.get(3)?,
            tags,
            statement: row.get(5)?,
            samples,
        })
    })?;
    rows.next().transpose()
}

#[derive(Debug, Clone)]
pub struct SolutionRow {
    pub id: i64,
    pub problem_id: String,
    pub platform: String,
    pub solved_at: DateTime<Utc>,
    pub time_taken_seconds: Option<u32>,
    pub execution_time_ms: Option<u32>,
    pub memory_used_kb: Option<u32>,
    pub attempts: u32,
    pub rating: Option<u32>,
    pub tags: Vec<String>,
    pub language: Option<String>,
    pub commit_hash: Option<String>,
}

pub fn insert_solution(
    conn: &Connection,
    problem_id: &str,
    platform: &str,
    time_taken: Option<u32>,
    exec_time: Option<u32>,
    mem_kb: Option<u32>,
    attempts: u32,
    rating: Option<u32>,
    tags: &[String],
    language: Option<&str>,
    commit_hash: Option<&str>,
) -> SqlResult<i64> {
    let tags_json = serde_json::to_string(tags).unwrap_or_else(|_| "[]".to_string());
    conn.execute(
        "INSERT INTO solutions
         (problem_id, platform, solved_at, time_taken_seconds, execution_time_ms,
          memory_used_kb, attempts, rating, tags, language, commit_hash)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            problem_id,
            platform,
            Utc::now().to_rfc3339(),
            time_taken,
            exec_time,
            mem_kb,
            attempts,
            rating,
            tags_json,
            language,
            commit_hash,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_all_solutions(conn: &Connection) -> SqlResult<Vec<SolutionRow>> {
    let mut stmt = conn.prepare(
        "SELECT id, problem_id, platform, solved_at, time_taken_seconds, execution_time_ms,
                memory_used_kb, attempts, rating, tags, language, commit_hash
         FROM solutions ORDER BY solved_at DESC"
    )?;
    let rows = stmt.query_map([], |row| {
        let solved_at_str: String = row.get(3)?;
        let tags: Vec<String> = serde_json::from_str(&row.get::<_, String>(9)?).unwrap_or_default();
        Ok(SolutionRow {
            id: row.get(0)?,
            problem_id: row.get(1)?,
            platform: row.get(2)?,
            solved_at: solved_at_str.parse::<DateTime<Utc>>().unwrap_or_else(|_| Utc::now()),
            time_taken_seconds: row.get(4)?,
            execution_time_ms: row.get(5)?,
            memory_used_kb: row.get(6)?,
            attempts: row.get(7)?,
            rating: row.get(8)?,
            tags,
            language: row.get(10)?,
            commit_hash: row.get(11)?,
        })
    })?;
    rows.collect()
}

pub fn get_solutions_by_tags(conn: &Connection, tag: &str) -> SqlResult<Vec<SolutionRow>> {
    let like = format!("%\"{}\"%", tag);
    let mut stmt = conn.prepare(
        "SELECT id, problem_id, platform, solved_at, time_taken_seconds, execution_time_ms,
                memory_used_kb, attempts, rating, tags, language, commit_hash
         FROM solutions WHERE tags LIKE ?1 ORDER BY solved_at DESC"
    )?;
    let rows = stmt.query_map([&like], |row| {
        let solved_at_str: String = row.get(3)?;
        let tags: Vec<String> = serde_json::from_str(&row.get::<_, String>(9)?).unwrap_or_default();
        Ok(SolutionRow {
            id: row.get(0)?,
            problem_id: row.get(1)?,
            platform: row.get(2)?,
            solved_at: solved_at_str.parse::<DateTime<Utc>>().unwrap_or_else(|_| Utc::now()),
            time_taken_seconds: row.get(4)?,
            execution_time_ms: row.get(5)?,
            memory_used_kb: row.get(6)?,
            attempts: row.get(7)?,
            rating: row.get(8)?,
            tags,
            language: row.get(10)?,
            commit_hash: row.get(11)?,
        })
    })?;
    rows.collect()
}
