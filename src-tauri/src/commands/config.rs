use std::fs;

use serde::{Deserialize, Serialize};

use crate::{Result, state::LayoutKind, utils::assets_dir};

const CONFIG_VERSION: u32 = 1;
const LAYOUTS_VERSION: u32 = 4;
const PAGES_VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
struct Versioned<T> {
    version: u32,
    data: T,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Plan {
    pub title: String,
    pub price: u32,
    pub copies: u8,
    pub popular: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Addon {
    pub title: String,
    pub price: u32,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    plans: Vec<Plan>,
    digital: Addon,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LayoutData {
    pub kind: LayoutKind,
    pub disabled: bool,
    pub title: String,
}

#[tauri::command]
pub fn save_config(config: Config) -> Result<()> {
    let mut path = assets_dir();
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;

    path.push("config.json");

    let wrapped = Versioned { version: CONFIG_VERSION, data: config };

    let json = serde_json::to_string_pretty(&wrapped).map_err(|e| e.to_string())?;

    fs::write(path, json).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_or_init_config(defaults: Config) -> Result<Config> {
    let mut path = assets_dir();
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;

    path.push("config.json");

    if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;

        if let Ok(parsed) = serde_json::from_str::<Versioned<Config>>(&content) {
            if parsed.version == CONFIG_VERSION {
                return Ok(parsed.data);
            }
        }

        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }

    let wrapped = Versioned { version: CONFIG_VERSION, data: defaults.clone() };

    let json = serde_json::to_string_pretty(&wrapped).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;

    Ok(defaults)
}

#[tauri::command]
pub fn save_layouts(layouts: Vec<LayoutData>) -> Result<()> {
    let mut path = assets_dir();
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;

    path.push("layouts.json");

    let wrapped = Versioned { version: LAYOUTS_VERSION, data: layouts };

    let json = serde_json::to_string_pretty(&wrapped).map_err(|e| e.to_string())?;

    fs::write(path, json).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_or_init_layouts(defaults: Vec<LayoutData>) -> Result<Vec<LayoutData>> {
    let mut path = assets_dir();
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

    let wrapped = Versioned { version: LAYOUTS_VERSION, data: defaults.clone() };

    let json = serde_json::to_string_pretty(&wrapped).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;

    Ok(defaults)
}

#[tauri::command]
pub fn save_pages(pages: u64) -> Result<()> {
    let mut path = assets_dir();
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;

    path.push("pages.json");

    let wrapped = Versioned { version: PAGES_VERSION, data: pages };

    let json = serde_json::to_string_pretty(&wrapped).map_err(|e| e.to_string())?;

    fs::write(path, json).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_or_init_pages(default: u64) -> Result<u64> {
    let mut path = assets_dir();
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;

    path.push("pages.json");

    if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;

        if let Ok(parsed) = serde_json::from_str::<Versioned<u64>>(&content) {
            if parsed.version == PAGES_VERSION {
                return Ok(parsed.data);
            }
        }

        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }

    let wrapped = Versioned { version: PAGES_VERSION, data: default };

    let json = serde_json::to_string_pretty(&wrapped).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;

    Ok(default)
}
