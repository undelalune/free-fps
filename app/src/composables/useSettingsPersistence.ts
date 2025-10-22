import {ref, Ref, watch} from 'vue';
import {isTauri} from '@tauri-apps/api/core';

export function useSettingsPersistence<T extends Record<string, any>>(
    initialValues: T,
    autoSave = true
) {
    const settings: { [K in keyof T]: Ref<T[K]> } = {} as any;

    for (const key in initialValues) {
        settings[key] = ref(initialValues[key]) as any;
    }

    const loadAllSettings = async () => {
        if (!isTauri()) return;

        const {initSettingsStore, loadSettings} = await import('@/stores/persistentStore');
        await initSettingsStore();

        for (const key in settings) {
            settings[key].value = await loadSettings(key, initialValues[key]);
        }
    };

    const saveSetting = async (key: string, value: any) => {
        if (!isTauri()) return;

        const {saveSettings} = await import('@/stores/persistentStore');
        await saveSettings(key, value);
    };

    if (autoSave) {
        for (const key in settings) {
            watch(settings[key], async (newValue) => {
                await saveSetting(key, newValue);
            });
        }
    }


    return {
        ...settings,
        loadAllSettings,
        saveSetting,
    };
}
