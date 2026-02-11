import {nextTick} from 'vue';
import {i18n, Locale} from '@/i18n';
import {useSettingsPersistence} from '@/composables/useSettingsPersistence';

export function useSettingsStore() {
    const {
        useDarkTheme,
        userLocale,
        loadAllSettings,

        inputFolder,
        outputFolder,
        targetFps,
        useCustomFps,
        customFps,
        keepAudio,
        audioBitrate,
        useCustomVideoQuality,
        videoQuality,
        cpuLimit,
        useGpu,
    } = useSettingsPersistence({
        useDarkTheme: true,
        userLocale: 'en' as Locale,

        inputFolder: '',
        outputFolder: '',
        targetFps: 30,
        useCustomFps: false,
        customFps: 30,
        keepAudio: false,
        audioBitrate: 192,
        useCustomVideoQuality: false,
        videoQuality: 16,
        cpuLimit: 75,
        useGpu: false,
    });

    const setLocale = (loc: Locale) => {
        userLocale.value = loc;
        i18n.locale.value = loc;
    };

    const resetDefaults = async (onDone?: () => void) => {
        useDarkTheme.value = true;

        inputFolder.value = '';
        outputFolder.value = '';
        targetFps.value = 30;
        useCustomFps.value = false;
        customFps.value = 30;
        keepAudio.value = false;
        audioBitrate.value = 192;
        useCustomVideoQuality.value = false;
        videoQuality.value = 16;
        cpuLimit.value = 75;
        useGpu.value = false;

        userLocale.value = 'en';
        setLocale(userLocale.value);

        await nextTick();
        if (onDone) onDone();
    };

    return {
        // refs
        useDarkTheme,
        userLocale,
        inputFolder,
        outputFolder,
        targetFps,
        useCustomFps,
        customFps,
        keepAudio,
        audioBitrate,
        useCustomVideoQuality,
        videoQuality,
        cpuLimit,
        useGpu,

        // actions
        loadAllSettings,
        resetDefaults,
        setLocale,
    };
}
