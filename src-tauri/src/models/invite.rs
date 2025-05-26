use crate::AppState;
use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};
use tauri::State;
//use rand::random;


#[derive(Serialize, Deserialize, Clone, Debug)] // Add Clone
pub struct Invitee {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct NewInvitee {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[tauri::command]
pub fn create_invite(state: State<AppState>, invite: NewInvitee) -> Result<i32, String> {
    let conn = state.db.lock().unwrap();
    let id = rand::random::<i32>();

    conn.execute(
        "INSERT INTO invite (firstname, lastname, email) VALUES (?1, ?2, ?3)",
        (&invite.firstname, &invite.lastname, &invite.email),
    )
    .map_err(|e| format!("BACKEND: Failed to create invite: {}", e.to_string()))?;

    Ok(id)
}

#[tauri::command]
pub fn get_all_invite(state: State<AppState>) -> Result<Vec<Invitee>, String> {
    let conn = state.db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, firstname, lastname, email FROM invite")
        .map_err(|e| e.to_string())?;
    
    let invite_iter = stmt.query_map([], |row| {
        Ok(Invitee {
            id: row.get(0)?,
            firstname: row.get(1)?,
            lastname: row.get(2)?,
            email: row.get(3)?,
        })
    })
    .map_err(|e: rusqlite::Error| e.to_string())?;

    let result: Result<Vec<Invitee>, _> = invite_iter.collect();
    result.map_err(|e: rusqlite::Error| e.to_string())
}


#[tauri::command]
pub fn get_invite(id: i32, state: State<AppState>) -> Result<Invitee, String> {
    let conn = state.db.lock().unwrap();
    let invite = conn
        .query_row(
            "SELECT id, firstname, lastname, mail FROM invite WHERE id = ?1",
            [id],
            |row| {
                Ok(Invitee {
                    id: row.get(0)?,
                    firstname: row.get(1)?,
                    lastname: row.get(2)?,
                    email: row.get(3)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;
    Ok(invite)
}

#[tauri::command]
pub fn update_invite(
    id: i32,
    invite: NewInvitee,
    state: State<AppState>,
) -> Result<String, String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;
    
    // Check if email already exists for a different invite
    let existing_invite = conn
        .query_row(
            "SELECT id FROM invite WHERE email = ?1 AND id != ?2",
            params![invite.email, id],
            |row| row.get::<_, i32>(0)
        );

    match existing_invite {
        Ok(_) => return Ok("Email already exists for another invite".to_string()),
        Err(rusqlite::Error::QueryReturnedNoRows) => (), // Email doesn't exist, continue
        Err(e) => return Err(format!("Database error: {}", e.to_string())),
    }
    
    conn.execute(
        "UPDATE invite SET firstname = ?1, lastname = ?2, email = ?3 WHERE id = ?4",
        params![
            invite.firstname,
            invite.lastname,
            invite.email,
            id
        ],
    )
    .map_err(|e| format!("BACKEND: Failed to update invite: {}", e.to_string()))?;
    Ok("Invitee updated successfully".to_string())
}

#[tauri::command]
pub fn delete_invite(ids: Vec<i32>, state: State<AppState>) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| e.to_string())?;

    // If the input array is empty, return early with success
    if ids.is_empty() {
        return Ok(());
    }

    // Create a parameterized IN clause, e.g., "(?, ?, ?)"
    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
    let query = format!("DELETE FROM invite WHERE id IN ({})", placeholders);

    // Convert Vec<i32> to Vec<&dyn rusqlite::ToSql> for params
    let params: Vec<&dyn rusqlite::ToSql> = ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();

    conn.execute(&query, &params[..])
        .map_err(|e| format!("BACKEND: Failed to delete invite: {}", e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub fn get_invite_soutenances(
    invite_id: i32,
    state: State<AppState>,
) -> Result<Vec<(i32, String)>, String> {
    let conn = state.db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT soutenance_id, role FROM invite_soutenance WHERE invite_id = ?1")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([invite_id], |row| Ok((row.get(0)?, row.get(1)?)))
        .map_err(|e| e.to_string())?;
    let result: Result<Vec<(i32, String)>, _> = rows.collect();
    result.map_err(|e| e.to_string())
}
