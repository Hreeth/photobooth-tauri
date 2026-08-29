import { Config } from "../Contexts/DataContext";
import { invoke } from "@tauri-apps/api/core";
import { LayoutData } from "../types";

import type { Options } from "../types"

// lifecycle
export async function startSession(options: Options): Promise<number> {
    return await invoke<number>("start_session", { options })
}

export async function resetSession() {
    await invoke("reset_session")
}

// camera
export async function startCamera() {
    await invoke("start_camera")
}

export async function stopCamera() {
    await invoke("stop_camera")
}

export async function capture(): Promise<string> {
    return await invoke<string>("capture")
}

export async function retake() {
    await invoke("retake")
}

export async function acceptPhoto(): Promise<boolean> {
    return await invoke<boolean>("accept_photo")
}

// imaging
export async function processSession() {
    await invoke("process_session")
}

// print
export async function print() {
    await invoke("print")
}

// config
export async function saveConfig(config: Config) {
    await invoke("save_config", { config })
}
export async function getOrInitConfig(defaults: Config) {
    return await invoke<Config>("get_or_init_config", { defaults });
}

export async function saveLayouts(layouts: LayoutData[]) {
    await invoke("save_layouts", { layouts })
}

export async function getOrInitLayouts(defaults: LayoutData[]) {
    return await invoke<LayoutData[]>("get_or_init_layouts", { defaults });
}

export async function savePages(pages: number) {
    await invoke("save_pages", { pages })
}

export async function getOrInitPages() {
    return await invoke<number>("get_or_init_pages", { default: 0 });
}