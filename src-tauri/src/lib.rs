mod models;

use models::{etudiant, invite, jury, jury_soutenance, invite_soutenance, pfe, classroom, soutenance, specialite, user};
use rusqlite::Connection;
use serde::Serialize;
use std::sync::Mutex;
use std::path::PathBuf;
use tauri::State;
use uuid::Uuid;

#[derive(Serialize)]
struct LoginResponse {
    access_token: String,
    message: String,
}

pub struct AppState {
    pub db: Mutex<Connection>,
}

fn get_db_path() -> PathBuf {
    let mut path = dirs::data_dir().expect("Failed to get data directory");
    path.push("gestion_soutenances_db");
    path.push("gestion_soutenances.db");
    path
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_path = get_db_path();
    std::fs::create_dir_all(db_path.parent().unwrap()).expect("Failed to create db directory");
    let conn = Connection::open(&db_path).expect("Failed to open database");

    conn.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL, email TEXT NOT NULL UNIQUE, password TEXT NOT NULL)", []).expect("Failed to create users table");
    
    conn.execute(
        "INSERT INTO users (username, email, password) 
        SELECT 'Admin', 'admin@example.com', 'admin'
        WHERE NOT EXISTS (SELECT 1 FROM users WHERE email = 'admin@example.com')",
        [],
    ).expect("Failed to insert admin user");
    
    conn.execute("CREATE TABLE IF NOT EXISTS etudiant (id INTEGER PRIMARY KEY AUTOINCREMENT, firstname TEXT NOT NULL, lastname TEXT NOT NULL, address TEXT NOT NULL, specialite_id INTEGER NOT NULL, soutenance_id INTEGER)", []).expect("Failed to create etudiant table");    
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS invite (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            firstname TEXT NOT NULL,
            lastname TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        )",
        [],
    )
    .expect("Failed to create invite table");
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS jury (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            firstname TEXT NOT NULL,
            lastname TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        )",
        [],
    )
    .expect("Failed to create jury table");

    conn.execute("CREATE TABLE IF NOT EXISTS jury_soutenance (jury_id INTEGER, soutenance_id INTEGER, role TEXT, PRIMARY KEY (jury_id, soutenance_id))", []).expect("Failed to create jury_soutenance table");
    
    conn.execute("CREATE TABLE IF NOT EXISTS pfe (id INTEGER PRIMARY KEY AUTOINCREMENT, specialite_id INTEGER)", []).expect("Failed to create pfe table");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS classroom (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT)",
        [],
    )
    .expect("Failed to create classroom table");
    
    conn.execute("CREATE TABLE IF NOT EXISTS soutenance (id INTEGER PRIMARY KEY AUTOINCREMENT, date TEXT, hour TEXT, specialite_id INTEGER, classroom_id INTEGER, pfe TEXT)", []).expect("Failed to create soutenance table");
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS specialite (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT)",
        [],
    )
    .expect("Failed to create specialite table");
    
    // Insert initial specialite data
    conn.execute(
        "INSERT INTO specialite (id, name) 
        SELECT 1, 'Licence Génie Industriel'
        WHERE NOT EXISTS (SELECT 1 FROM specialite WHERE id = 1)",
        [],
    ).expect("Failed to insert specialite 1");
    
    conn.execute(
        "INSERT INTO specialite (id, name) 
        SELECT 2, 'Licence Génie Informatique'
        WHERE NOT EXISTS (SELECT 1 FROM specialite WHERE id = 2)",
        [],
    ).expect("Failed to insert specialite 2");
    
    conn.execute(
        "INSERT INTO specialite (id, name) 
        SELECT 3, 'Mastére Industrie v4.0'
        WHERE NOT EXISTS (SELECT 1 FROM specialite WHERE id = 3)",
        [],
    ).expect("Failed to insert specialite 3");
    
    conn.execute(
        "INSERT INTO specialite (id, name) 
        SELECT 4, 'Génie Civil'
        WHERE NOT EXISTS (SELECT 1 FROM specialite WHERE id = 4)",
        [],
    ).expect("Failed to insert specialite 4");
    
    conn.execute(
        "INSERT INTO specialite (id, name) 
        SELECT 5, 'Génie Procédés'
        WHERE NOT EXISTS (SELECT 1 FROM specialite WHERE id = 5)",
        [],
    ).expect("Failed to insert specialite 5");
    
    conn.execute(
        "INSERT INTO specialite (id, name) 
        SELECT 6, 'Génie Télécommunication'
        WHERE NOT EXISTS (SELECT 1 FROM specialite WHERE id = 6)",
        [],
    ).expect("Failed to insert specialite 6");
    
    conn.execute(
        "INSERT INTO specialite (id, name) 
        SELECT 7, 'Génie Industriel'
        WHERE NOT EXISTS (SELECT 1 FROM specialite WHERE id = 7)",
        [],
    ).expect("Failed to insert specialite 7");
    
    conn.execute(
        "INSERT INTO specialite (id, name) 
        SELECT 8, 'Génie Informatique'
        WHERE NOT EXISTS (SELECT 1 FROM specialite WHERE id = 8)",
        [],
    ).expect("Failed to insert specialite 8");
    
    conn.execute(
        "INSERT INTO specialite (id, name) 
        SELECT 9, 'Génie Mécanique'
        WHERE NOT EXISTS (SELECT 1 FROM specialite WHERE id = 9)",
        [],
    ).expect("Failed to insert specialite 9");
    
    conn.execute("CREATE TABLE IF NOT EXISTS invite_soutenance (invite_id INTEGER, soutenance_id INTEGER, PRIMARY KEY (invite_id, soutenance_id))", []).expect("Failed to create invite_soutenance table");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            db: Mutex::new(conn),
        })
        .invoke_handler(tauri::generate_handler![
            login,
            etudiant::create_student,
            etudiant::get_student,
            etudiant::get_students_by_department,
            etudiant::update_student,
            etudiant::delete_students,
            etudiant::get_specialite_students,
            invite::create_invite,
            invite::get_invite,
            invite::update_invite,
            invite::delete_invite,
            invite::get_invite_soutenances,
            invite::get_all_invite,

            invite_soutenance::create_invite_soutenance,
            invite_soutenance::get_invite_soutenance,
            invite_soutenance::update_invite_soutenance,
            invite_soutenance::delete_invite_soutenance,

            jury_soutenance::create_jury_soutenance,
            jury_soutenance::get_jury_soutenance,
            jury_soutenance::update_jury_soutenance,
            jury_soutenance::delete_jury_soutenance,
            jury::create_jury,
            jury::get_jury,
            jury::get_all_jury,
            jury::update_jury,
            jury::delete_jury,
            jury::get_jury_soutenances,
            pfe::create_pfe,
            pfe::get_pfe,
            pfe::update_pfe,
            pfe::delete_pfe,
            classroom::create_classroom,    
            classroom::get_all_classrooms,
            classroom::get_classroom,
            classroom::update_classroom,
            classroom::delete_classrooms,
            classroom::get_classroom_soutenances,
            soutenance::create_soutenance,
            soutenance::get_soutenance,
            soutenance::update_soutenance,
            soutenance::delete_soutenance,
            soutenance::get_soutenance_students,
            soutenance::get_soutenance_jurys,
            soutenance::get_soutenance_invites,
            soutenance::get_specialite_soutenances,
            specialite::create_specialite,
            specialite::get_specialite,
            specialite::update_specialite,
            specialite::delete_specialite,
            
            specialite::get_specialite_pfes,
            user::create_user,
            user::get_user,
            user::update_user,
            user::delete_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn login(email: String, password: String, state: State<AppState>) -> Result<LoginResponse, String> {
    let conn = state.db.lock().unwrap();
    //let hashed_password = user::hash_password(&password);
    let mut stmt = conn
        .prepare("SELECT id FROM users WHERE email = ?1 AND password = ?2")
        .map_err(|e| e.to_string())?;
    let user_exists = stmt
        .exists([&email, &password])
        .map_err(|e| e.to_string())?;
    if user_exists {
        let token = Uuid::new_v4().to_string();
        Ok(LoginResponse {
            access_token: token,
            message: "Login successful".to_string(),
        })
    } 
    else {
        //Err("Invalid credentials".to_string())
        Ok(LoginResponse {
            access_token: "".to_string(),
            message: "Invalid credentials".to_string(),
        })
    }
}
