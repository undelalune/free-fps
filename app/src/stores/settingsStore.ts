import { nextTick } from 'vue';
import { i18n, Locale } from '@/i18n';
import { useSettingsPersistence } from '@/composables/useSettingsPersistence';
import type { FfToolsStatus } from '@/types';

export function useSettingsStore() {
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
    } = useSettingsPersistence({
        useDarkTheme: true,
        ffmpegPath: '',
        ffprobePath: '',
        ffmpegInstalledVersion: '',
        ffprobeInstalledVersion: '',
        ffmpegUseInstalled: false,
        ffprobeUseInstalled: false,
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
    });

    const setLocale = (loc: Locale) => {
        userLocale.value = loc;
        i18n.locale.value = loc;
    };

    const resetDefaults = async (onDone?: () => void) => {
        useDarkTheme.value = true;
        ffmpegPath.value = '';
        ffprobePath.value = '';
        ffmpegInstalledVersion.value = '';
        ffprobeInstalledVersion.value = '';
        ffmpegUseInstalled.value = false;
        ffprobeUseInstalled.value = false;

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

        userLocale.value = 'en';
        setLocale(userLocale.value);

        await nextTick();
        if (onDone) onDone();
    };

    function applyToolsStatus(res: FfToolsStatus) {
        const prevFfmpegVer = ffmpegInstalledVersion.value || '';
        const prevFfprobeVer = ffprobeInstalledVersion.value || '';
        const prevFfmpegPath = ffmpegPath.value || '';
        const prevFfprobePath = ffprobePath.value || '';

        const curFfmpegVer = res?.ffmpeg || '';
        const curFfprobeVer = res?.ffprobe || '';

        const isFirstRun = !prevFfmpegVer && !prevFfprobeVer && !prevFfmpegPath && !prevFfprobePath;
        const changed = prevFfmpegVer !== curFfmpegVer || prevFfprobeVer !== curFfprobeVer;

        ffmpegInstalledVersion.value = curFfmpegVer;
        ffprobeInstalledVersion.value = curFfprobeVer;

        if (!curFfmpegVer) ffmpegUseInstalled.value = false;
        if (!curFfprobeVer) ffprobeUseInstalled.value = false;

        return {
            isFirstRun,
            changed,
            showDialog: isFirstRun || changed,
            curFfmpegVer,
            curFfprobeVer,
        };
    }

    return {
        // refs
        useDarkTheme,
        ffmpegPath,
        ffprobePath,
        ffmpegInstalledVersion,
        ffprobeInstalledVersion,
        ffmpegUseInstalled,
        ffprobeUseInstalled,
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

        // actions
        loadAllSettings,
        resetDefaults,
        setLocale,
        applyToolsStatus,
    };
}
