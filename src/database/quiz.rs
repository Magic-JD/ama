use crate::cli::command::{AddArgs, CreateArgs, EvalArgs};
use crate::state::quiz::{Quiz, Round};
use crate::utils::file_system::database_path;
use chrono::{DateTime, Duration, Utc};
use once_cell::sync::Lazy;
use rusqlite::{Connection, params};
use std::sync::{Mutex, MutexGuard};

static CONNECTION: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open(database_path()).expect("Failed to open database");
    conn.execute_batch(
        "
    PRAGMA foreign_keys = ON;

    CREATE TABLE IF NOT EXISTS quiz (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS round (
        id INTEGER PRIMARY KEY,
        question TEXT NOT NULL,
        answer TEXT NOT NULL,
        test_after DATETIME,
        quiz_id INTEGER NOT NULL,
        FOREIGN KEY (quiz_id)
            REFERENCES quiz(id)
            ON DELETE CASCADE
            ON UPDATE CASCADE
    );

    CREATE TABLE IF NOT EXISTS app_state (
        id INTEGER PRIMARY KEY CHECK (id = 1),
        current_quiz_id INTEGER,
        FOREIGN KEY (current_quiz_id)
            REFERENCES quiz(id)
            ON DELETE SET NULL
    );
    INSERT OR IGNORE INTO app_state (id)
    VALUES (1);

    CREATE INDEX IF NOT EXISTS idx_round_quiz_test_after
    ON round (quiz_id, test_after);
    ",
    )
    .expect("Failed to initialize database");
    Mutex::new(conn)
});

pub fn create_new_quiz(args: CreateArgs) -> Quiz {
    let conn = CONNECTION.lock().unwrap();
    let new_quiz_id = conn
        .execute("INSERT INTO quiz (name) VALUES (?)", [args.quiz_name])
        .unwrap();
    conn.execute(
        "UPDATE app_state
     SET current_quiz_id = ?
     WHERE id = 1",
        [new_quiz_id],
    )
    .unwrap();
    load_quiz(&conn, new_quiz_id as i64)
}

pub fn add_round_to_current_quiz(args: AddArgs) -> Option<Quiz> {
    let conn = CONNECTION.lock().unwrap();
    let quiz_id = match current_quiz_id(&conn) {
        Some(id) => id,
        None => return None,
    };
    conn.execute("INSERT INTO round (question, answer, test_after, quiz_id) VALUES (?, ?, datetime('now'), ?)", params![
        args.question,
        args.answer,
        quiz_id,
    ]).unwrap();
    Some(load_quiz(&conn, quiz_id))
}

pub fn resolve_current_round(args: EvalArgs) -> Option<Quiz> {
    let conn = CONNECTION.lock().unwrap();
    let quiz_id = match current_quiz_id(&conn) {
        Some(id) => id,
        None => return None,
    };
    let round_id: Option<i64> = conn
        .query_row(
            "
            SELECT id
            FROM round
            WHERE quiz_id = ?
            AND (test_after IS NULL OR test_after <= CURRENT_TIMESTAMP)
            ORDER BY test_after ASC
            LIMIT 1;
",
            [quiz_id],
            |row| row.get(0),
        )
        .unwrap();
    let added_duration = if args.pass {
        Duration::days(1)
    } else {
        Duration::seconds(5)
    };
    let test_after: DateTime<Utc> = Utc::now() + added_duration;
    conn.execute(
        "UPDATE round SET test_after = ? WHERE id = ?",
        params![test_after, round_id,],
    )
    .unwrap();
    Some(load_quiz(&conn, quiz_id))
}

pub fn load_current_quiz() -> Option<Quiz> {
    let conn = CONNECTION.lock().unwrap();
    let quiz_id = current_quiz_id(&conn);
    match quiz_id {
        Some(id) => Some(load_quiz(&conn, id)),
        None => None,
    }
}

fn current_quiz_id(conn: &MutexGuard<Connection>) -> Option<i64> {
    let quiz_id: Option<i64> = conn
        .query_row(
            "SELECT current_quiz_id FROM app_state WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .unwrap();
    quiz_id
}

fn load_quiz(conn: &Connection, id: i64) -> Quiz {
    let (id, name): (i64, String) = conn
        .query_row("SELECT id, name FROM quiz WHERE id = ?", [id], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .unwrap();

    let mut stmt = conn
        .prepare(
            "
            SELECT id, question, answer, test_after
            FROM round
            WHERE quiz_id = ?
            AND (test_after IS NULL OR test_after <= CURRENT_TIMESTAMP)
            ORDER BY test_after ASC;
",
        )
        .unwrap();

    let rounds = stmt
        .query_map([id], |row| {
            Ok(Round {
                question: row.get(1)?,
                answer: row.get(2)?,
            })
        })
        .unwrap()
        .flatten()
        .collect::<Vec<Round>>();

    Quiz { name, rounds }
}
