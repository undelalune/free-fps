import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { VideoFile, VideoConversionParams, ConversionProgress } from '@/types';

export const videoAPI = {
    async getVideoFiles(folderPath: string): Promise<VideoFile[]> {
        return await invoke<VideoFile[]>('get_video_files', { folderPath });
    },

    async getVideoThumbnail(path: string): Promise<string> {
        return await invoke<string>('get_video_thumbnail', { path });
    },

    async convertVideos(params: VideoConversionParams): Promise<string> {
        return await invoke<string>('convert_videos', { params });
    },

    async cancelConversion(): Promise<void> {
        await invoke('cancel_conversion');
    },

    onConversionProgress(callback: (progress: ConversionProgress) => void) {
        return listen<ConversionProgress>('conversion-progress', (event) => {
            callback(event.payload);
        });
    }
};
