// typescript
// src/store/persistentStore.ts
import { Store } from '@tauri-apps/plugin-store';
import { shallowRef, markRaw } from 'vue';

export const settingsStore = shallowRef<Store | null>(null);

export async function initSettingsStore(): Promise<void> {
    let store = await Store.get('settings.json');
    if (!store) {
        store = await Store.load('settings.json');
    }
    // Prevent Vue from proxying the instance (required for classes with private fields)
    settingsStore.value = markRaw(store);
}

// Helper functions to work with the store
export async function loadSettings<T>(key: string, defaultValue: T): Promise<T> {
    const store = settingsStore.value;
    if (!store) return defaultValue;
    const value = await store.get<T>(key);
    return value !== undefined ? value : defaultValue;
}

export async function saveSettings<T>(key: string, value: T): Promise<void> {
    const store = settingsStore.value;
    if (!store) return;
    await store.set(key, value);
    // Optionally persist immediately (autoSave may already handle this)
    // await store.save();
}

export async function clearSettings(): Promise<void> {
    const store = settingsStore.value;
    if (!store) return;
    await store.clear();
    await store.save();
}
