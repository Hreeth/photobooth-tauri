use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Plan {
    pub title: String,
    pub price: u32,
    pub strips: u8,
    pub popular: bool
}

#[tauri::command]
pub fn save_pricing(directory: String, plans: Vec<Plan>) -> Result<(), String> {
    let mut path = PathBuf::from(directory);

    path.push("Memorabooth");
    fs::create_dir_all(&path)
        .map_err(|e| e.to_string())?;

    path.push("plans.json");

    let json = serde_json::to_string_pretty(&plans)
        .map_err(|e| e.to_string())?;

    fs::write(path, json)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_or_init_pricing(directory: String, defaults: Vec<Plan>) -> Result<Vec<Plan>, String> {
    let mut path = PathBuf::from(directory);
    path.push("Memorabooth");
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;

    path.push("plans.json");

    // Already exists → just return it
    if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let parsed: Vec<Plan> = serde_json::from_str(&content).map_err(|e| e.to_string())?;
        return Ok(parsed);
    }

    // Doesn't exist → write defaults
    let json = serde_json::to_string_pretty(&defaults).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;

    Ok(defaults)
}