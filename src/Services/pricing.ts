import { documentDir } from "@tauri-apps/api/path";
import { Plan } from "../Contexts/DataContext";
import { invoke } from "@tauri-apps/api/core";

export async function savePricing(plans: Plan[]) {
    const dir = await documentDir()

    await invoke("save_pricing", { directory: dir, plans })
}

export async function getOrInitPricing(defaults: Plan[]) {
    const dir = await documentDir()

    return await invoke<Plan[]>("get_or_init_pricing", { directory: dir, defaults });
}