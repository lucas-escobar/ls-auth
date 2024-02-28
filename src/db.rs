use rusqlite::{Connection, Error, Result};

use crate::models::User;

const DB_FILE: &str = "auth.db";
const CREATE_USERS_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY,
        username TEXT UNIQUE NOT NULL,
        password_hash TEXT NOT NULL,
        email TEXT UNIQUE NOT NULL
    )
";

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open(DB_FILE)?;
    conn.execute(CREATE_USERS_TABLE, [])?;
    Ok(conn)
}

pub fn create_user(
    conn: &Connection,
    username: &str,
    password_hash: &str,
    email: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO users (username, password_hash, email) VALUES (?1, ?2, ?3)",
        &[username, password_hash, email],
    )?;
    Ok(())
}

pub fn get_user_by_username(conn: &Connection, username: &str) -> Result<Option<User>> {
    let mut stmt = conn.prepare("SELECT * FROM users WHERE username = ?1")?;
    let mut rows = stmt.query(&[username])?;
    if let Some(row) = rows.next()? {
        let user = User {
            id: row.get(0)?,
            username: row.get(1)?,
            password_hash: row.get(2)?,
            email: row.get(3)?,
        };
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

pub fn update_user_email(conn: &Connection, username: &str, email: &str) -> Result<()> {
    conn.execute(
        "UPDATE users SET email = ?1 WHERE username = ?2",
        &[email, username],
    )?;
    Ok(())
}

pub fn delete_user(conn: &Connection, username: &str) -> Result<()> {
    conn.execute("DELETE FROM users WHERE username = ?1", &[username])?;
    Ok(())
}

pub fn verify_credentials(conn: &Connection, username: &str, password_hash: &str) -> Result<bool> {
    let mut stmt =
        conn.prepare("SELECT COUNT(*) FROM users WHERE username = ?1 AND password_hash = ?2")?;
    let count: i32 = stmt.query_row(&[username, password_hash], |row| row.get(0))?;
    Ok(count > 0)
}

fn handle_db_error(err: Error) {
    eprintln!("Database error: {}", err);
}
