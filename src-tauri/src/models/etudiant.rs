use crate::AppState;
use rusqlite::Result;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize, Clone, Debug)] // Added Clone and Debug
pub struct Etudiant {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub address: String,
    pub specialite_id: i32,
    pub soutenance_id: Option<i32>,
}

#[tauri::command]
pub fn create_student(
    firstname: String,
    lastname: String,
    address: String,
    specialite_id: i32,
    soutenance_id: Option<i32>,
    state: State<AppState>,
) -> Result<i32, String> {
    let conn = state.db.lock().unwrap();
    conn.execute(
        "INSERT INTO etudiant (firstname, lastname, address, specialite_id, soutenance_id) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![firstname, lastname, address, specialite_id, soutenance_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid() as i32)
}

#[tauri::command]
pub fn get_student(id: i32, state: State<AppState>) -> Result<Etudiant, String> {
    let conn = state.db.lock().unwrap();
    let etudiant = conn
        .query_row(
            "SELECT id, firstname, lastname, address, specialite_id, soutenance_id FROM etudiant WHERE id = ?1",
            [id],
            |row| {
                Ok(Etudiant {
                    id: row.get(0)?,
                    firstname: row.get(1)?,
                    lastname: row.get(2)?,
                    address: row.get(3)?,
                    specialite_id: row.get(4)?,
                    soutenance_id: row.get(5)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;
    Ok(etudiant)
}


#[tauri::command]
pub fn update_student(
    id: i32,
    firstname: String,
    lastname: String,
    address: String,
    specialite_id: i32,
    soutenance_id: i32,
    state: State<AppState>,
) -> Result<Etudiant, String> { // Changed return type here
    {
        let conn = state.db.lock().unwrap();
        // conn.execute(
        //     "UPDATE etudiant SET firstname = ?1, lastname = ?2, address = ?3, soutenance_id = ?4 WHERE id = ?5",
        //     rusqlite::params![firstname, lastname, address, soutenance_id, id],
        // )
        conn.execute(
            "UPDATE etudiant SET firstname = ?1, lastname = ?2, address = ?3, specialite_id = ?4, soutenance_id = ?5 WHERE id = ?6",
            rusqlite::params![firstname, lastname, address, specialite_id, soutenance_id, id],
        )
        .map_err(|e| e.to_string())?;
        // conn is dropped here as it goes out of scope
    }
    
    get_student(id, state)
}

#[tauri::command]
pub fn delete_students(ids: Vec<i32>, state: State<AppState>) -> Result<(), String> {
    let mut conn = state.db.lock().unwrap(); // Add `mut` here
    
    // Start a transaction to ensure all deletions succeed or fail together

    let tx = conn.transaction().map_err(|e| e.to_string())?;
    
    for id in ids {
        tx.execute("DELETE FROM etudiant WHERE id = ?1", [id])
            .map_err(|e| e.to_string())?;
    }
    
    // Commit the transaction
    tx.commit().map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub fn get_students_by_department(department_id: i32, state: State<AppState>) -> Result<Vec<Etudiant>, String> {
    let conn = state.db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, firstname, lastname, address, specialite_id, soutenance_id FROM etudiant WHERE specialite_id = ?1")
        .map_err(|e| e.to_string())?;
    let etudiant_iter = stmt
        .query_map([department_id], |row| {
            Ok(Etudiant {
                id: row.get(0)?,
                firstname: row.get(1)?,
                lastname: row.get(2)?,
                address: row.get(3)?,
                specialite_id: row.get(4)?,
                soutenance_id: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut students = Vec::new();
    for etudiant in etudiant_iter {
        students.push(etudiant.map_err(|e| e.to_string())?);
    }
    Ok(students)
}


#[tauri::command]
pub fn get_specialite_students(
    specialite_id: i32,
    state: State<AppState>,
) -> Result<Vec<Etudiant>, String> {
    let conn = state.db.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, firstname, lastname, address, specialite_id, soutenance_id FROM etudiant WHERE specialite_id = ?1")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([specialite_id], |row| {
            Ok(Etudiant {
                id: row.get(0)?,
                firstname: row.get(1)?,
                lastname: row.get(2)?,
                address: row.get(3)?,
                specialite_id: row.get(4)?,
                soutenance_id: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?;
    let result: Result<Vec<Etudiant>, _> = rows.collect();
    result.map_err(|e| e.to_string())
}
