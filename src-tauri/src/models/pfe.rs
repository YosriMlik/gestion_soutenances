use crate::AppState;
use rusqlite::Result;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct Pfe {
    pub id: i32,
    pub specialite_id: i32,
}

#[tauri::command]
pub fn create_pfe(specialite_id: i32, state: State<AppState>) -> Result<i32, String> {
    let conn = state.db.lock().unwrap();
    conn.execute(
        "INSERT INTO pfe (specialite_id) VALUES (?1)",
        [specialite_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid() as i32)
}

#[tauri::command]
pub fn get_pfe(id: i32, state: State<AppState>) -> Result<Pfe, String> {
    let conn = state.db.lock().unwrap();
    let pfe = conn
        .query_row(
            "SELECT id, specialite_id FROM pfe WHERE id = ?1",
            [id],
            |row| {
                Ok(Pfe {
                    id: row.get(0)?,
                    specialite_id: row.get(1)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;
    Ok(pfe)
}

#[tauri::command]
pub fn update_pfe(id: i32, specialite_id: i32, state: State<AppState>) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    conn.execute(
        "UPDATE pfe SET specialite_id = ?1 WHERE id = ?2",
        [specialite_id, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_pfe(id: i32, state: State<AppState>) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    conn.execute("DELETE FROM pfe WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
