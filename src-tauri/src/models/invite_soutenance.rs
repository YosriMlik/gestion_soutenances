use crate::AppState;
use rusqlite::Result;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct InviteSoutenance {
    pub invite_id: i32,
    pub soutenance_id: i32,
}

#[tauri::command]
pub fn create_invite_soutenance(
    invite_id: i32,
    soutenance_id: i32,
    state: State<AppState>,
) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    conn.execute(
        "INSERT INTO invite_soutenance (invite_id, soutenance_id) VALUES (?1, ?2)",
        rusqlite::params![invite_id, soutenance_id], // Use params! and as_str()
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_invite_soutenance(
    invite_id: i32,
    soutenance_id: i32,
    state: State<AppState>,
) -> Result<InviteSoutenance, String> {
    let conn = state.db.lock().unwrap();
    let pivot = conn.query_row(
        "SELECT invite_id, soutenance_id FROM invite_soutenance WHERE invite_id = ?1 AND soutenance_id = ?2",
        [invite_id, soutenance_id],
        |row| Ok(InviteSoutenance {
            invite_id: row.get(0)?,
            soutenance_id: row.get(1)?,
        }),
    ).map_err(|e| e.to_string())?;
    Ok(pivot)
}

#[tauri::command]
pub fn update_invite_soutenance(
    invite_id: i32,
    soutenance_id: i32,
    state: State<AppState>,
) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    conn.execute(
        "UPDATE invite_soutenance SET invite_id = ?1, soutenance_id = ?2 WHERE invite_id = ?1 AND soutenance_id = ?2",
        rusqlite::params![invite_id, soutenance_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_invite_soutenance(
    invite_id: i32,
    soutenance_id: i32,
    state: State<AppState>,
) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    conn.execute(
        "DELETE FROM invite_soutenance WHERE invite_id = ?1 AND soutenance_id = ?2",
        [invite_id, soutenance_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
