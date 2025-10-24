import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import {
    VideoFile,
    VideoConversionParams,
    ConversionProgress,
    FfToolsStatus,
    ToolCheckParams,
    ThumbnailParams
} from '@/types';

export const videoAPI = {
    async getVideoFiles(folderPath: string): Promise<VideoFile[]> {
        return await invoke<VideoFile[]>('get_video_files', { folderPath });
    },

    async getVideoThumbnail(params: ThumbnailParams): Promise<string> {
        return await invoke<string>('get_video_thumbnail', { params });
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

export const toolsAPI = {
    async checkToolsInstalled(): Promise<FfToolsStatus> {
        return await invoke<FfToolsStatus>('check_ff_tools');
    },

    async checkToolSelected(params: ToolCheckParams): Promise<boolean> {
        return await invoke<boolean>('check_ff_tool_selected', { params });
    }
};

