import {ref, watch} from 'vue';
import {useStore} from '@/stores';
import {toolsAPI} from "@/api/tauri.ts";

export function useFFToolsDialog() {
    const store = useStore();
    const showFFPopup = ref(false);

    async function checkFFInstalled() {
        try {
            const res = await toolsAPI.checkToolsInstalled()
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