import { defineStore } from 'pinia';
import { i18n, Locale } from '@/i18n';
import { darkThemeOverrides, lightThemeOverrides } from '@/themes';
import { computed, ref } from 'vue';
import { useSettingsStore } from './settingsStore';
import type { VideoFile } from '@/types';

export const useStore = defineStore('index', () => {
    const settings = useSettingsStore();
    const {
        useDarkTheme,
        ffmpegPath,
        ffprobePath,
        ffmpegInstalledVersion,
        ffprobeInstalledVersion,
        ffmpegUseInstalled,
        ffprobeUseInstalled,
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
        resetDefaults,
        setLocale,
        applyToolsStatus,
    } = settings;

    const isDarkTheme = computed(() => useDarkTheme?.value);
    const storeInitialized = ref(false);
    const videoFiles = ref<VideoFile[]>([]);
    const folderScanning = ref(false);
    const processing = ref(false);
    const statusMessage = ref('');
    const showHelp = ref(false);
    const heartIsBeating = ref(false);

    const theme = computed(() => (useDarkTheme.value ? darkThemeOverrides : lightThemeOverrides));
    const locale = computed(() => i18n.locale.value as Locale);

    const switchTheme = () => {
        useDarkTheme.value = !useDarkTheme.value;
        document.body.style.backgroundColor = theme.value.common?.bodyColor || '';
    };

    const init = async () => {
        await loadAllSettings();
        storeInitialized.value = true;
        setLocale(userLocale.value);
    };

    const resetSettings = async () => {
        storeInitialized.value = false;
        await resetDefaults(async () => {
            storeInitialized.value = true;
        });
    }

    return {
        // state
        theme,
        ffmpegPath,
        ffprobePath,
        ffmpegInstalledVersion,
        ffprobeInstalledVersion,
        ffmpegUseInstalled,
        ffprobeUseInstalled,
        isDarkTheme,
        locale,
        storeInitialized,
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
        videoFiles,
        folderScanning,
        processing,
        statusMessage,
        showHelp,
        heartIsBeating,
        // actions
        init,
        switchTheme,
        setLocale,
        resetSettings,
        applyToolsStatus,
    };
});
