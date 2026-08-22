import { documentDir } from "@tauri-apps/api/path";
import { Config } from "../Contexts/DataContext";
import { invoke } from "@tauri-apps/api/core";
import { LayoutData } from "../types";

export async function saveConfig(config: Config) {
    const dir = await documentDir()

    await invoke("save_config", { directory: dir, config })
}
export async function getOrInitConfig(defaults: Config) {
    const dir = await documentDir()

    return await invoke<Config>("get_or_init_config", { directory: dir, defaults });
}

export async function saveLayouts(layouts: LayoutData[]) {
    const dir = await documentDir()

    await invoke("save_layouts", { directory: dir, layouts })
}

export async function getOrInitLayouts(defaults: LayoutData[]) {
    const dir = await documentDir()

    return await invoke<LayoutData[]>("get_or_init_layouts", { directory: dir, defaults });
}

export async function savePages(pages: number) {
    const dir = await documentDir()

    await invoke("save_pages", { directory: dir, pages })
}

export async function getOrInitPages() {
    const dir = await documentDir()

    return await invoke<number>("get_or_init_pages", { directory: dir, default: 0 });
}