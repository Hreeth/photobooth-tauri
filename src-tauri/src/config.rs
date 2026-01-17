use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::imaging::Layout;

const PRICING_VERSION: u32 = 1;
const LAYOUTS_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
struct Versioned<T> {
    version: u32,
    data: T
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Plan {
    pub title: String,
    pub price: u32,
    pub copies: u8,
    pub popular: bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LayoutData {
    pub kind: Layout,
    pub disabled: bool
}

#[tauri::command]
pub fn save_pricing(directory: String, plans: Vec<Plan>) -> Result<(), String> {
    let mut path = PathBuf::from(directory);

    path.push("Memorabooth");
    fs::create_dir_all(&path)
        .map_err(|e| e.to_string())?;

    path.push("plans.json");

    let wrapped = Versioned {
        version: PRICING_VERSION,
        data: plans
    };

    let json = serde_json::to_string_pretty(&wrapped)
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

    if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;

        if let Ok(parsed) = serde_json::from_str::<Versioned<Vec<Plan>>>(&content) {
            if parsed.version == PRICING_VERSION {
                return Ok(parsed.data)
            }
        }

        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }

    let wrapped = Versioned {
        version: PRICING_VERSION,
        data: defaults.clone(),
    };

    let json = serde_json::to_string_pretty(&wrapped).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;

    Ok(defaults)
}

#[tauri::command]
pub fn save_layouts(directory: String, layouts: Vec<LayoutData>) -> Result<(), String> {
    let mut path = PathBuf::from(directory);

    path.push("Memorabooth");
    fs::create_dir_all(&path)
        .map_err(|e| e.to_string())?;

    path.push("layouts.json");

    let wrapped = Versioned {
        version: LAYOUTS_VERSION,
        data: layouts
    };

    let json = serde_json::to_string_pretty(&wrapped)
        .map_err(|e| e.to_string())?;

    fs::write(path, json)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_or_init_layouts(directory: String, defaults: Vec<LayoutData>) -> Result<Vec<LayoutData>, String> {
    let mut path = PathBuf::from(directory);
    path.push("Memorabooth");
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;

    path.push("layouts.json");

    if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;

        if let Ok(parsed) = serde_json::from_str::<Versioned<Vec<LayoutData>>>(&content) {
            if parsed.version == LAYOUTS_VERSION {
                return Ok(parsed.data);
            }
        }
        
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }

    let wrapped = Versioned {
        version: LAYOUTS_VERSION,
        data: defaults.clone(),
    };

    let json = serde_json::to_string_pretty(&wrapped).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;

    Ok(defaults)
}