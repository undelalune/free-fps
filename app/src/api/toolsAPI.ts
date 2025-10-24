import {invoke} from '@tauri-apps/api/core';
import {FfToolsStatus, ToolCheckParams} from '@/types';

export const toolsAPI = {
    async checkToolsInstalled(): Promise<FfToolsStatus> {
        return await invoke<FfToolsStatus>('check_ff_tools');
    },

    async checkToolSelected(params: ToolCheckParams): Promise<boolean> {
        return await invoke<boolean>('check_ff_tool_selected', { params });
    }
};
