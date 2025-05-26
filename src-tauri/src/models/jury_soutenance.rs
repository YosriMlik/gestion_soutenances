use crate::AppState;
use rusqlite::Result;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct JurySoutenance {
    pub jury_id: i32,
    pub soutenance_id: i32,
    pub role: String,
}

#[tauri::command]
pub fn create_jury_soutenance(
    jury_id: i32,
    soutenance_id: i32,
    role: String,
    state: State<AppState>,
) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    conn.execute(
        "INSERT INTO jury_soutenance (jury_id, soutenance_id, role) VALUES (?1, ?2, ?3)",
        rusqlite::params![jury_id, soutenance_id, role.as_str()], // Use params! and as_str()
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_jury_soutenance(
    jury_id: i32,
    soutenance_id: i32,
    state: State<AppState>,
) -> Result<JurySoutenance, String> {
    let conn = state.db.lock().unwrap();
    let pivot = conn.query_row(
        "SELECT jury_id, soutenance_id, role FROM jury_soutenance WHERE jury_id = ?1 AND soutenance_id = ?2",
        [jury_id, soutenance_id],
        |row| Ok(JurySoutenance {
            jury_id: row.get(0)?,
            soutenance_id: row.get(1)?,
            role: row.get(2)?,
        }),
    ).map_err(|e| e.to_string())?;
    Ok(pivot)
}

#[tauri::command]
pub fn update_jury_soutenance(
    jury_id: i32,
    soutenance_id: i32,
    role: String,
    state: State<AppState>,
) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    conn.execute(
        "UPDATE jury_soutenance SET role = ?1 WHERE jury_id = ?2 AND soutenance_id = ?3",
        rusqlite::params![&role, &jury_id.to_string(), &soutenance_id.to_string()],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_jury_soutenance(
    jury_id: i32,
    soutenance_id: i32,
    state: State<AppState>,
) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    conn.execute(
        "DELETE FROM jury_soutenance WHERE jury_id = ?1 AND soutenance_id = ?2",
        [jury_id, soutenance_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
