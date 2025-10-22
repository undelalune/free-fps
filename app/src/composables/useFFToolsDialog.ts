import {ref, watch} from 'vue';
import {useStore} from '@/stores';
import type {FfToolsStatus} from '@/types';
import {invoke} from "@tauri-apps/api/core";

export function useFFToolsDialog() {
    const store = useStore();
    const showFFPopup = ref(false);

    async function checkFFInstalled() {
        try {
            const res = await invoke<FfToolsStatus>('check_ff_tools');
            const {showDialog} = store.applyToolsStatus(res);
            if (showDialog) showFFPopup.value = true;
        } catch (e) {
            console.log(e);
        }
    }

    watch(
        () => store.storeInitialized,
        ready => {
            if (ready) checkFFInstalled();
        },
        {immediate: true}
    );

    return {showFFPopup};
}