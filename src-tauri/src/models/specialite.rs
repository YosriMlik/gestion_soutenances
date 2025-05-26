use crate::AppState;
use rusqlite::Result;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize)]
pub struct Specialite {
    pub id: i32,
    pub name: String
}

#[tauri::command]
pub fn create_specialite(state: State<AppState>) -> Result<i32, String> {
    let conn = state.db.lock().unwrap();
    conn.execute("INSERT INTO specialite DEFAULT VALUES", [])
        .map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid() as i32)
}

#[tauri::command]
pub fn get_specialite(id: i32, state: State<AppState>) -> Result<Specialite, String> {
    let conn = state.db.lock().unwrap();
    let specialite = conn
        .query_row("SELECT id, name FROM specialite WHERE id = ?1", [id], |row| {
            Ok(Specialite {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;
    Ok(specialite)
}

#[tauri::command]
pub fn update_specialite(_id: i32, _state: State<AppState>) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub fn delete_specialite(id: i32, state: State<AppState>) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    conn.execute("DELETE FROM specialite WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}


#[tauri::command]
pub fn get_specialite_pfes(specialite_id: i32, state: State<AppState>) -> Result<Vec<i32>, String> {
    let conn = state.db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id FROM pfe WHERE specialite_id = ?1")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([specialite_id], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    let result: Result<Vec<i32>, _> = rows.collect();
    result.map_err(|e| e.to_string())
}
