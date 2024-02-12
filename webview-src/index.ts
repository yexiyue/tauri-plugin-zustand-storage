import { invoke } from "@tauri-apps/api/core";

export async function getItem(key: string) {
  return await invoke<string | null>("plugin:zustand-storage|get_item", {
    key,
  });
}

export async function setItem(key: string, value: string) {
  return await invoke<void>("plugin:zustand-storage|set_item", {
    key,
    value,
  });
}

export async function removeItem(key: string) {
  return await invoke<void>("plugin:zustand-storage|remove_item", {
    key,
  });
}

export const ZustandStorage = {
  getItem,
  setItem,
  removeItem,
};
