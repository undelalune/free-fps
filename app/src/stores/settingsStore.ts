import {nextTick} from 'vue';
import {i18n, Locale} from '@/i18n';
import {useSettingsPersistence} from '@/composables/useSettingsPersistence';

const DEFAULTS = {
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
};

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
    } = useSettingsPersistence(DEFAULTS);

    const setLocale = (loc: Locale) => {
        userLocale.value = loc;
        i18n.locale.value = loc;
    };

    const resetDefaults = async (onDone?: () => void) => {
        useDarkTheme.value = DEFAULTS.useDarkTheme;
        inputFolder.value = DEFAULTS.inputFolder;
        outputFolder.value = DEFAULTS.outputFolder;
        targetFps.value = DEFAULTS.targetFps;
        useCustomFps.value = DEFAULTS.useCustomFps;
        customFps.value = DEFAULTS.customFps;
        keepAudio.value = DEFAULTS.keepAudio;
        audioBitrate.value = DEFAULTS.audioBitrate;
        useCustomVideoQuality.value = DEFAULTS.useCustomVideoQuality;
        videoQuality.value = DEFAULTS.videoQuality;
        cpuLimit.value = DEFAULTS.cpuLimit;
        useGpu.value = DEFAULTS.useGpu;

        userLocale.value = DEFAULTS.userLocale;
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
