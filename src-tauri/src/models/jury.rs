use crate::AppState;
use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};
use tauri::State;
//use rand::random;


#[derive(Serialize, Deserialize, Clone, Debug)] // Added Clone and Debug
pub struct Jury {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct NewJury {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[tauri::command]
pub fn create_jury(state: State<AppState>, jury: NewJury) -> Result<i32, String> {
    let conn = state.db.lock().unwrap();
    let id = rand::random::<i32>();

    conn.execute(
        "INSERT INTO jury (firstname, lastname, email) VALUES (?1, ?2, ?3)",
        (&jury.firstname, &jury.lastname, &jury.email),
    )
    .map_err(|e| format!("BACKEND: Failed to create jury: {}", e.to_string()))?;

    Ok(id)
}

#[tauri::command]
pub fn get_all_jury(state: State<AppState>) -> Result<Vec<Jury>, String> {
    let conn = state.db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, firstname, lastname, email FROM jury")
        .map_err(|e| e.to_string())?;
    
    let jury_iter = stmt.query_map([], |row| {
        Ok(Jury {
            id: row.get(0)?,
            firstname: row.get(1)?,
            lastname: row.get(2)?,
            email: row.get(3)?,
        })
    })
    .map_err(|e: rusqlite::Error| e.to_string())?;

    let result: Result<Vec<Jury>, _> = jury_iter.collect();
    result.map_err(|e: rusqlite::Error| e.to_string())
}


#[tauri::command]
pub fn get_jury(id: i32, state: State<AppState>) -> Result<Jury, String> {
    let conn = state.db.lock().unwrap();
    let jury = conn
        .query_row(
            "SELECT id, firstname, lastname, mail FROM jury WHERE id = ?1",
            [id],
            |row| {
                Ok(Jury {
                    id: row.get(0)?,
                    firstname: row.get(1)?,
                    lastname: row.get(2)?,
                    email: row.get(3)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;
    Ok(jury)
}

#[tauri::command]
pub fn update_jury(
    id: i32,
    jury: NewJury,
    state: State<AppState>,
) -> Result<String, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    // Check if email already exists for a different jury
    let existing_jury = conn
        .query_row(
            "SELECT id FROM jury WHERE email = ?1 AND id != ?2",
            params![jury.email, id],
            |row| row.get::<_, i32>(0)
        );

    match existing_jury {
        Ok(_) => return Ok("Email already exists for another jury".to_string()),
        Err(rusqlite::Error::QueryReturnedNoRows) => (), // Email doesn't exist, continue
        Err(e) => return Err(format!("Database error: {}", e.to_string())),
    }
    
    conn.execute(
        "UPDATE jury SET firstname = ?1, lastname = ?2, email = ?3 WHERE id = ?4",
        params![
            jury.firstname,
            jury.lastname,
            jury.email,
            id
        ],
    )
    .map_err(|e| format!("BACKEND: Failed to update jury: {}", e.to_string()))?;
    Ok("Jury updated successfully".to_string())
}

#[tauri::command]
pub fn delete_jury(ids: Vec<i32>, state: State<AppState>) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    // If the input array is empty, return early with success
    if ids.is_empty() {
        return Ok(());
    }

    // Create a parameterized IN clause, e.g., "(?, ?, ?)"
    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
    let query = format!("DELETE FROM jury WHERE id IN ({})", placeholders);

    // Convert Vec<i32> to Vec<&dyn rusqlite::ToSql> for params
    let params: Vec<&dyn rusqlite::ToSql> = ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();

    conn.execute(&query, &params[..])
        .map_err(|e| format!("BACKEND: Failed to delete jury: {}", e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn get_jury_soutenances(
    jury_id: i32,
    state: State<AppState>,
) -> Result<Vec<(i32, String)>, String> {
    let conn = state.db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT soutenance_id, role FROM jury_soutenance WHERE jury_id = ?1")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([jury_id], |row| Ok((row.get(0)?, row.get(1)?)))
        .map_err(|e| e.to_string())?;
    let result: Result<Vec<(i32, String)>, _> = rows.collect();
    result.map_err(|e| e.to_string())
}
