use rusqlite::{params, Connection, Result as SqlResult};

use crate::models::Flashcard;
use crate::sm2::{self, Rating, Sm2Result};

/// Create the schema if it does not exist.
pub fn init_db(conn: &Connection) -> SqlResult<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS flashcards (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            front       TEXT NOT NULL,
            back        TEXT NOT NULL,
            ease_factor REAL NOT NULL DEFAULT 2.5,
            interval    INTEGER NOT NULL DEFAULT 0,
            repetitions INTEGER NOT NULL DEFAULT 0,
            next_review TEXT NOT NULL DEFAULT (date('now')),
            created_at  TEXT NOT NULL DEFAULT (datetime('now'))
        );",
    )
}

/// Insert demo cards if the table is empty.
pub fn seed_cards(conn: &Connection) -> SqlResult<()> {
    let count: i64 =
        conn.query_row("SELECT COUNT(*) FROM flashcards", [], |r| r.get(0))?;
    if count > 0 {
        return Ok(());
    }

    let cards: &[(&str, &str)] = &[
        ("Hello", "Hola"),
        ("Goodbye", "Adiós"),
        ("Thank you", "Gracias"),
        ("Please", "Por favor"),
        ("Good morning", "Buenos días"),
        ("Good night", "Buenas noches"),
        ("How are you?", "¿Cómo estás?"),
        ("I'm sorry", "Lo siento"),
        ("Water", "Agua"),
        ("Food", "Comida"),
        ("Friend", "Amigo"),
        ("House", "Casa"),
        ("Book", "Libro"),
        ("Time", "Tiempo"),
        ("Love", "Amor"),
        ("Family", "Familia"),
        ("Work", "Trabajo"),
        ("School", "Escuela"),
        ("Happy", "Feliz"),
        ("Beautiful", "Hermoso"),
    ];

    for (front, back) in cards {
        conn.execute(
            "INSERT INTO flashcards (front, back) VALUES (?1, ?2)",
            params![front, back],
        )?;
    }

    Ok(())
}

/// Count cards due for review (next_review <= today).
pub fn get_due_count(conn: &Connection) -> SqlResult<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM flashcards WHERE next_review <= date('now')",
        [],
        |r| r.get(0),
    )
}

/// Count all cards.
pub fn get_total_count(conn: &Connection) -> SqlResult<i64> {
    conn.query_row("SELECT COUNT(*) FROM flashcards", [], |r| r.get(0))
}

/// Get the single most-overdue card, or None if none are due.
pub fn get_due_card(conn: &Connection) -> SqlResult<Option<Flashcard>> {
    let mut stmt = conn.prepare(
        "SELECT id, front, back, ease_factor, interval, repetitions, next_review, created_at
         FROM flashcards
         WHERE next_review <= date('now')
         ORDER BY next_review ASC
         LIMIT 1",
    )?;

    let mut rows = stmt.query_map([], row_to_card)?;
    match rows.next() {
        Some(Ok(card)) => Ok(Some(card)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}

/// Get a card by id.
pub fn get_card(conn: &Connection, id: i64) -> SqlResult<Option<Flashcard>> {
    let mut stmt = conn.prepare(
        "SELECT id, front, back, ease_factor, interval, repetitions, next_review, created_at
         FROM flashcards WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(params![id], row_to_card)?;
    match rows.next() {
        Some(Ok(card)) => Ok(Some(card)),
        Some(Err(e)) => Err(e),
        None => Ok(None),
    }
}

/// List all cards, newest first.
pub fn get_all_cards(conn: &Connection) -> SqlResult<Vec<Flashcard>> {
    let mut stmt = conn.prepare(
        "SELECT id, front, back, ease_factor, interval, repetitions, next_review, created_at
         FROM flashcards
         ORDER BY created_at DESC",
    )?;
    let rows = stmt.query_map([], row_to_card)?;
    rows.collect()
}

/// Create a new card and return it.
pub fn create_card(conn: &Connection, front: &str, back: &str) -> SqlResult<Flashcard> {
    conn.execute(
        "INSERT INTO flashcards (front, back) VALUES (?1, ?2)",
        params![front, back],
    )?;
    let id = conn.last_insert_rowid();
    get_card(conn, id).map(|o| o.unwrap())
}

/// Update a card's front/back text.
pub fn update_card(conn: &Connection, id: i64, front: &str, back: &str) -> SqlResult<()> {
    conn.execute(
        "UPDATE flashcards SET front = ?1, back = ?2 WHERE id = ?3",
        params![front, back, id],
    )?;
    Ok(())
}

/// Delete a card by id.
pub fn delete_card(conn: &Connection, id: i64) -> SqlResult<()> {
    conn.execute("DELETE FROM flashcards WHERE id = ?1", params![id])?;
    Ok(())
}

/// Apply SM-2 result to the card row.
pub fn apply_sm2(conn: &Connection, card_id: i64, result: &Sm2Result) -> SqlResult<()> {
    conn.execute(
        "UPDATE flashcards
         SET ease_factor = ?1, interval = ?2, repetitions = ?3, next_review = ?4
         WHERE id = ?5",
        params![
            result.ease_factor,
            result.interval,
            result.repetitions,
            result.next_review,
            card_id,
        ],
    )?;
    Ok(())
}

/// Process an answer: compute SM-2, persist, and return the updated card.
pub fn answer_card(
    conn: &Connection,
    card_id: i64,
    rating: Rating,
) -> Result<Flashcard, crate::AppError> {
    let card = get_card(conn, card_id)?.ok_or(crate::AppError::NotFound)?;
    let result = sm2::calculate(card.ease_factor, card.interval, card.repetitions, rating);
    apply_sm2(conn, card_id, &result)?;
    get_card(conn, card_id)?.ok_or(crate::AppError::NotFound)
}

fn row_to_card(row: &rusqlite::Row) -> SqlResult<Flashcard> {
    Ok(Flashcard {
        id: row.get(0)?,
        front: row.get(1)?,
        back: row.get(2)?,
        ease_factor: row.get(3)?,
        interval: row.get(4)?,
        repetitions: row.get(5)?,
        next_review: row.get(6)?,
        created_at: row.get(7)?,
    })
}
