use crate::AppState;
use rusqlite::Result;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize, Clone, Debug)] // Add Clone
pub struct Salle {
    pub id: i32,
    pub name: String, // Added name field
}

#[tauri::command]
pub fn create_classroom(name: String, state: State<AppState>) -> Result<i32, String> { // Added name parameter
    let conn = state.db.lock().unwrap();
    conn.execute("INSERT INTO classroom (name) VALUES (?1)", [name]) // Modified SQL to insert name
        .map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid() as i32)
}

#[tauri::command]
pub fn get_classroom(id: i32, state: State<AppState>) -> Result<Salle, String> {
    let conn = state.db.lock().unwrap();
    let salle = conn
        .query_row("SELECT id, name FROM classroom WHERE id = ?1", [id], |row| { // Modified SQL to select name
            Ok(Salle { 
                id: row.get(0)?,
                name: row.get(1)? // Get name from row
            })
        })
        .map_err(|e| e.to_string())?;
    Ok(salle)
}

#[tauri::command]
pub fn update_classroom(id: i32, name: String, state: State<AppState>) -> Result<(), String> { // Added name parameter
    let conn = state.db.lock().unwrap();
    conn.execute("UPDATE classroom SET name = ?1 WHERE id = ?2", [name, id.to_string()]) // Implemented update logic
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_classrooms(ids: Vec<i32>, state: State<AppState>) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    // If the input array is empty, return early with success
    if ids.is_empty() {
        return Ok(());
    }

    // Create a parameterized IN clause, e.g., "(?, ?, ?)"
    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
    let query = format!("DELETE FROM classroom WHERE id IN ({})", placeholders);

    // Convert Vec<i32> to Vec<&dyn rusqlite::ToSql> for params
    let params: Vec<&dyn rusqlite::ToSql> = ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();

    conn.execute(&query, &params[..])
        .map_err(|e| format!("BACKEND: Failed to delete classroom: {}", e.to_string()))?;

    Ok(())
}

// New function to get all classrooms
#[tauri::command]
pub fn get_all_classrooms(state: State<AppState>) -> Result<Vec<Salle>, String> {
    let conn = state.db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, name FROM classroom")
        .map_err(|e| e.to_string())?;
    let salle_iter = stmt
        .query_map([], |row| {
            Ok(Salle {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut salles = Vec::new();
    for salle in salle_iter {
        salles.push(salle.map_err(|e| e.to_string())?);
    }
    Ok(salles)
}

#[tauri::command]
pub fn get_classroom_soutenances(salle_id: i32, state: State<AppState>) -> Result<Vec<i32>, String> {
    let conn = state.db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id FROM soutenance WHERE salle_id = ?1")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([salle_id], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    let result: Result<Vec<i32>, _> = rows.collect();
    result.map_err(|e| e.to_string())
}
