use crate::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::models::{classroom::Salle, invite::Invitee, jury::Jury, etudiant::Etudiant};
use rusqlite::{Result, Row, ffi};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JurySoutenanceDetails {
    #[serde(flatten)]
    pub jury: Jury,
    pub role: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Soutenance {
    pub id: i32,
    pub date: Option<String>,
    pub hour: Option<String>,
    pub specialite_id: i32,
    pub pfe: Option<String>,
    pub classroom: Option<Salle>,
    pub juries: Vec<JurySoutenanceDetails>,
    pub invitees: Vec<Invitee>,
    pub students: Vec<Etudiant>,
}

#[tauri::command]
pub fn create_soutenance(
    date: Option<String>,
    hour: Option<String>,
    specialite_id: i32,
    classroom_id: Option<i32>, // Changed to Option<i32> to handle NULL
    pfe: Option<String>,
    state: State<AppState>,
) -> Result<Soutenance, String> {
    let conn = state.db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;

    conn.execute(
        "INSERT INTO soutenance (date, hour, specialite_id, classroom_id, pfe) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![date, hour, specialite_id, classroom_id, pfe],
    )
    .map_err(|e| format!("Failed to insert soutenance: {}", e))?;

    let id = conn.last_insert_rowid() as i32;

    // Fetch the classroom object
    let classroom = if let Some(cid) = classroom_id {
        conn.query_row(
            "SELECT id, name FROM classroom WHERE id = ?1",
            [cid],
            |row| Ok(Salle {
                id: row.get(0)?,
                name: row.get(1)?,
            }),
        )
        .map_err(|e| format!("Failed to fetch classroom: {}", e))
        .ok() // Convert error to None if classroom not found
    } else {
        None
    };

    Ok(Soutenance {
        id,
        date,
        hour,
        specialite_id,
        pfe,
        classroom,
        juries: Vec::new(), // Initially empty
        invitees: Vec::new(),
        students: Vec::new(),
    })
}

#[tauri::command]
pub fn get_soutenance(id: i32, state: State<AppState>) -> Result<Soutenance, String> {
    let conn = state.db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;

    let query = r#"
        SELECT 
            s.id, s.date, s.hour, s.specialite_id, s.pfe,
            c.id AS classroom_id, c.name AS classroom_name,
            COALESCE((
                SELECT json_group_array(
                    json_object(
                        'id', j.id,
                        'firstname', j.firstname,
                        'lastname', j.lastname,
                        'email', j.email,
                        'role', js.role
                    )
                )
                FROM jury j
                JOIN jury_soutenance js ON j.id = js.jury_id
                WHERE js.soutenance_id = s.id
            ), '[]') AS juries,
            COALESCE((
                SELECT json_group_array(
                    json_object(
                        'id', i.id,
                        'firstname', i.firstname,
                        'lastname', i.lastname,
                        'email', i.email
                    )
                )
                FROM invite i
                JOIN invite_soutenance ins ON i.id = ins.invite_id
                WHERE ins.soutenance_id = s.id
            ), '[]') AS invitees,
            COALESCE((
                SELECT json_group_array(
                    json_object(
                        'id', e.id,
                        'firstname', e.firstname,
                        'lastname', e.lastname,
                        'address', e.address,
                        'specialite_id', e.specialite_id,
                        'soutenance_id', e.soutenance_id
                    )
                )
                FROM etudiant e
                WHERE e.soutenance_id = s.id
            ), '[]') AS students
        FROM soutenance s
        LEFT JOIN classroom c ON s.classroom_id = c.id
        WHERE s.id = ?1
    "#;

    let soutenance = conn
        .query_row(query, [id], |row: &Row| {
            let id: i32 = row.get(0)?;
            let date: Option<String> = row.get(1)?;
            let hour: Option<String> = row.get(2)?;
            let specialite_id: i32 = row.get(3)?;
            let pfe: Option<String> = row.get(4)?;

            let classroom = match row.get::<_, Option<i32>>(5)? {
                Some(cid) => Some(Salle {
                    id: cid,
                    name: row.get(6)?,
                }),
                None => None,
            };

            let juries_json: String = row.get(7)?;
            let juries: Vec<JurySoutenanceDetails> = serde_json::from_str(&juries_json)
                .map_err(|e| {
                    let err_msg = format!("Failed to parse juries JSON: {}", e);
                    rusqlite::Error::SqliteFailure(
                        ffi::Error::new(ffi::SQLITE_MISUSE),
                        Some(err_msg),
                    )
                })?;

            let invitees_json: String = row.get(8)?;
            let invitees: Vec<Invitee> = serde_json::from_str(&invitees_json)
                .map_err(|e| {
                    let err_msg = format!("Failed to parse invitees JSON: {}", e);
                    rusqlite::Error::SqliteFailure(
                        ffi::Error::new(ffi::SQLITE_MISUSE),
                        Some(err_msg),
                    )
                })?;

            let students_json: String = row.get(9)?;
            let students: Vec<Etudiant> = serde_json::from_str(&students_json)
                .map_err(|e| {
                    let err_msg = format!("Failed to parse students JSON: {}", e);
                    rusqlite::Error::SqliteFailure(
                        ffi::Error::new(ffi::SQLITE_MISUSE),
                        Some(err_msg),
                    )
                })?;

            Ok(Soutenance {
                id,
                date,
                hour,
                specialite_id,
                pfe,
                classroom,
                juries,
                invitees,
                students,
            })
        })
        .map_err(|e| format!("Failed to fetch soutenance: {}", e))?;

    Ok(soutenance)
}

#[tauri::command]
pub fn update_soutenance(
    id: i32,
    date: Option<String>,
    hour: Option<String>,
    specialite_id: i32,
    classroom_id: Option<i32>, // Changed to Option<i32>
    pfe: Option<String>,
    state: State<AppState>,
) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    conn.execute(
        "UPDATE soutenance SET date = ?1, hour = ?2, specialite_id = ?3, classroom_id = ?4, pfe = ?5 WHERE id = ?6",
        rusqlite::params![date, hour, specialite_id, classroom_id, pfe, id],
    )
    .map_err(|e| format!("Failed to update soutenance: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn delete_soutenance(id: i32, state: State<AppState>) -> Result<(), String> {
    let conn = state.db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    conn.execute("DELETE FROM soutenance WHERE id = ?1", [id])
        .map_err(|e| format!("Failed to delete soutenance: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn get_soutenance_students(
    soutenance_id: i32,
    state: State<AppState>,
) -> Result<Vec<i32>, String> {
    let conn = state.db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    let mut stmt = conn
        .prepare("SELECT id FROM etudiant WHERE soutenance_id = ?1")
        .map_err(|e| format!("Failed to prepare student query: {}", e))?;
    let rows = stmt
        .query_map([soutenance_id], |row| row.get(0))
        .map_err(|e| format!("Failed to query students: {}", e))?;
    let result: Result<Vec<i32>, _> = rows.collect();
    result.map_err(|e| format!("Failed to collect students: {}", e))
}

#[tauri::command]
pub fn get_soutenance_jurys(
    soutenance_id: i32,
    state: State<AppState>,
) -> Result<Vec<(i32, String)>, String> {
    let conn = state.db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    let mut stmt = conn
        .prepare("SELECT jury_id, role FROM jury_soutenance WHERE soutenance_id = ?1")
        .map_err(|e| format!("Failed to prepare jury query: {}", e))?;
    let rows = stmt
        .query_map([soutenance_id], |row| Ok((row.get(0)?, row.get(1)?)))
        .map_err(|e| format!("Failed to query juries: {}", e))?;
    let result: Result<Vec<(i32, String)>, _> = rows.collect();
    result.map_err(|e| format!("Failed to collect juries: {}", e))
}

#[tauri::command]
pub fn get_soutenance_invites(
    soutenance_id: i32,
    state: State<AppState>,
) -> Result<Vec<i32>, String> {
    let conn = state.db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;
    let mut stmt = conn
        .prepare("SELECT invite_id FROM invite_soutenance WHERE soutenance_id = ?1")
        .map_err(|e| format!("Failed to prepare invite query: {}", e))?;
    let rows = stmt
        .query_map([soutenance_id], |row| row.get(0))
        .map_err(|e| format!("Failed to query invitees: {}", e))?;
    let result: Result<Vec<i32>, _> = rows.collect();
    result.map_err(|e| format!("Failed to collect invitees: {}", e))
}

#[tauri::command]
pub fn get_specialite_soutenances(
    specialite_id: i32,
    state: State<AppState>,
) -> Result<Vec<Soutenance>, String> {
    let conn = state.db.lock().map_err(|e| format!("Failed to lock database: {}", e))?;

    let query = r#"
        SELECT 
            s.id, s.date, s.hour, s.specialite_id, s.pfe,
            c.id AS classroom_id, c.name AS classroom_name,
            COALESCE((
                SELECT json_group_array(
                    json_object(
                        'id', j.id,
                        'firstname', j.firstname,
                        'lastname', j.lastname,
                        'email', j.email,
                        'role', js.role
                    )
                )
                FROM jury j
                JOIN jury_soutenance js ON j.id = js.jury_id
                WHERE js.soutenance_id = s.id
            ), '[]') AS juries,
            COALESCE((
                SELECT json_group_array(
                    json_object(
                        'id', i.id,
                        'firstname', i.firstname,
                        'lastname', i.lastname,
                        'email', i.email
                    )
                )
                FROM invite i
                JOIN invite_soutenance ins ON i.id = ins.invite_id
                WHERE ins.soutenance_id = s.id
            ), '[]') AS invitees,
            COALESCE((
                SELECT json_group_array(
                    json_object(
                        'id', e.id,
                        'firstname', e.firstname,
                        'lastname', e.lastname,
                        'address', e.address,
                        'specialite_id', e.specialite_id,
                        'soutenance_id', e.soutenance_id
                    )
                )
                FROM etudiant e
                WHERE e.soutenance_id = s.id
            ), '[]') AS students
        FROM soutenance s
        LEFT JOIN classroom c ON s.classroom_id = c.id
        WHERE s.specialite_id = ?1
    "#;

    let mut stmt = conn
        .prepare(query)
        .map_err(|e| format!("Failed to prepare soutenance query: {}", e))?;

    let rows = stmt
        .query_map([specialite_id], |row| {
            let id: i32 = row.get(0)?;
            let date: Option<String> = row.get(1)?;
            let hour: Option<String> = row.get(2)?;
            let specialite_id: i32 = row.get(3)?;
            let pfe: Option<String> = row.get(4)?;

            let classroom = match row.get::<_, Option<i32>>(5)? {
                Some(cid) => Some(Salle {
                    id: cid,
                    name: row.get(6)?,
                }),
                None => None,
            };

            let juries_json: String = row.get(7)?;
            let juries: Vec<JurySoutenanceDetails> = serde_json::from_str(&juries_json)
                .map_err(|e| {
                    let err_msg = format!("Failed to parse juries JSON: {}", e);
                    rusqlite::Error::SqliteFailure(
                        ffi::Error::new(ffi::SQLITE_MISUSE),
                        Some(err_msg),
                    )
                })?;

            let invitees_json: String = row.get(8)?;
            let invitees: Vec<Invitee> = serde_json::from_str(&invitees_json)
                .map_err(|e| {
                    let err_msg = format!("Failed to parse invitees JSON: {}", e);
                    rusqlite::Error::SqliteFailure(
                        ffi::Error::new(ffi::SQLITE_MISUSE),
                        Some(err_msg),
                    )
                })?;

            let students_json: String = row.get(9)?;
            let students: Vec<Etudiant> = serde_json::from_str(&students_json)
                .map_err(|e| {
                    let err_msg = format!("Failed to parse students JSON: {}", e);
                    rusqlite::Error::SqliteFailure(
                        ffi::Error::new(ffi::SQLITE_MISUSE),
                        Some(err_msg),
                    )
                })?;

            Ok(Soutenance {
                id,
                date,
                hour,
                specialite_id,
                pfe,
                classroom,
                juries,
                invitees,
                students,
            })
        })
        .map_err(|e| format!("Failed to query soutenances: {}", e))?;

    let result: Result<Vec<Soutenance>, _> = rows.collect();
    result.map_err(|e| format!("Failed to collect soutenances: {}", e))
}
