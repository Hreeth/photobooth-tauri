import { documentDir } from "@tauri-apps/api/path";
import { LayoutData, Plan } from "../Contexts/DataContext";
import { invoke } from "@tauri-apps/api/core";

export async function savePricing(plans: Plan[]) {
    const dir = await documentDir()

    await invoke("save_pricing", { directory: dir, plans })
}
export async function getOrInitPricing(defaults: Plan[]) {
    const dir = await documentDir()

    return await invoke<Plan[]>("get_or_init_pricing", { directory: dir, defaults });
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