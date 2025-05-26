use crate::AppState;
use rusqlite::Result;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip)]
    #[allow(dead_code)] // Add this to suppress the warning
    pub password: String,
}

#[tauri::command]
pub fn create_user(
    name: String,
    email: String,
    password: String,
    state: State<AppState>,
) -> Result<i32, String> {
    let conn = state.db.lock().unwrap();
    let hashed_password = hash_password(&password);
    conn.execute(
        "INSERT INTO users (name, email, password) VALUES (?1, ?2, ?3)",
        [&name, &email, &hashed_password],
    )
    .map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid() as i32)
}

#[tauri::command]
pub fn get_user(id: i32, state: State<AppState>) -> Result<User, String> {
    let conn = state.db.lock().unwrap();
    let user = conn
        .query_row(
            "SELECT id, name, email, password FROM users WHERE id = ?1",
            [id],
            |row| {
                Ok(User {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    email: row.get(2)?,
                    password: row.get(3)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;
    Ok(user)
}

#[tauri::command]
pub fn update_user(
    id: i32,
    name: String,
    email: String,
    password: String,
    state: State<AppState>,
) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    let hashed_password = hash_password(&password);
    conn.execute(
        "UPDATE users SET name = ?1, email = ?2, password = ?3 WHERE id = ?4",
        [&name, &email, &hashed_password, &id.to_string()],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_user(id: i32, state: State<AppState>) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    conn.execute("DELETE FROM users WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    format!("{:x}", hasher.finalize())
}
