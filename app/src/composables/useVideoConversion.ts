import { ref, computed } from 'vue';
import { useStore } from '@/stores';
import { videoAPI } from '@/api/tauri.ts';
import { useI18n } from 'vue-i18n';
import { useMessage } from 'naive-ui';
import type { AppError } from '@/types';
import { ConversionStatus, ErrorCode } from '@/types';

export function useVideoConversion() {
    const store = useStore();
    const { t } = useI18n();
    const message = useMessage();

    const currentScanId = ref(0);
    const unlistenProgress = ref<(() => void) | null>(null);

    const selectedFiles = computed(() => store.videoFiles.filter(f => f.convert));
    const isFfmpegConfigured = computed(() =>
        (store.ffmpegUseInstalled && store.ffmpegInstalledVersion) ||
        (!store.ffmpegUseInstalled && store.ffmpegPath)
    );

    const showMsg = (type: 'success' | 'warning', key: string, duration = 5000) => {
        message[type](t(key), { duration, closable: true });
    };

    const scanFolder = async (folderPath?: string | null) => {
        const scanId = ++currentScanId.value;

        if (!folderPath) {
            store.videoFiles = [];
            return;
        }

        if (store.folderScanning) {
            await videoAPI.cancelConversion().catch(console.warn);
        }

        store.folderScanning = true;

        try {
            const files = await videoAPI.getVideoFiles(folderPath);

            if (scanId === currentScanId.value) {
                store.videoFiles = files.map(f => ({ ...f, convert: true, progress: 0 }));
            }
        } catch (error) {
            console.error('Scan failed:', error);
            if (scanId === currentScanId.value) {
                store.videoFiles = [];
            }
        } finally {
            if (scanId === currentScanId.value) {
                store.folderScanning = false;
            }
        }
    };

    const performConversion = async () => {
        if (selectedFiles.value.length === 0) {
            showMsg('warning', 'mainView.processing.noFilesSelected');
            return;
        }

        store.processing = true;

        try {
            await videoAPI.convertVideos({
                ffmpeg_path: store.ffmpegPath || '',
                ffprobe_path: store.ffprobePath || '',
                ffmpeg_use_installed: store.ffmpegUseInstalled,
                ffprobe_use_installed: store.ffprobeUseInstalled,
                input_folder: store.inputFolder,
                output_folder: store.outputFolder,
                target_fps: store.useCustomFps ? store.customFps : store.targetFps,
                cpu_limit: store.cpuLimit,
                keep_audio: store.keepAudio,
                audio_bitrate: store.audioBitrate,
                use_custom_video_quality: store.useCustomVideoQuality,
                video_quality: store.videoQuality,
                files: selectedFiles.value
                    .filter(f => f.status !== ConversionStatus.Success)
                    .map(f => f.path),
            });

            const hasErrors = selectedFiles.value.some(f =>
                f.convert && f.status !== ConversionStatus.Success
            );

            showMsg(
                hasErrors ? 'warning' : 'success',
                hasErrors ? 'mainView.processing.operationCompletedWithErrors' : 'mainView.processing.conversionCompleted',
                hasErrors ? 10000 : 5000
            );
        } catch (error) {
            const errorCode = (error as AppError)?.code || ErrorCode.None;
            showMsg('warning', `errors.${errorCode}`, 10000);
        } finally {
            store.processing = false;
        }
    };

    const cancelConversion = async () => {
        await videoAPI.cancelConversion().catch(console.warn);
        store.processing = false;
    };

    const setupProgressListener = async () => {
        unlistenProgress.value = await videoAPI.onConversionProgress((data) => {
            if (!data || data.status === ConversionStatus.Cancelled) return;

            const video = store.videoFiles.find(v => v.name === data.current_file);
            if (video) {
                video.progress = Math.floor(data.percentage) || 0;
                video.status = data.status;
            }
        });
    };

    const cleanup = () => unlistenProgress.value?.();

    return {
        isFfmpegConfigured,
        scanFolder,
        rescanFolder: () => scanFolder(store.inputFolder),
        performConversion,
        cancelConversion,
        setupProgressListener,
        cleanup,
    };
}