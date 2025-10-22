<template>
  <n-config-provider :theme-overrides="store.theme">
    <n-global-style/>
    <div id="app" class="app">
      <n-message-provider>
        <router-view/>
      </n-message-provider>
      <n-modal-provider>
        <FFFoundDialogProvider/>
        <HelpDialog/>
      </n-modal-provider>
    </div>
    <AppFooter/>
  </n-config-provider>
</template>

<script setup lang="ts">
import {useStore} from "@/stores";
import {onMounted} from "vue";
import FFFoundDialogProvider from "@/components/dialogs/FFFoundDialogProvider.vue";
import HelpDialog from "@/components/dialogs/HelpDialog.vue";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {confirm} from '@tauri-apps/plugin-dialog';
import {videoAPI} from "@/api/tauri.ts";
import {useI18n} from "vue-i18n";
import AppFooter from "@/components/AppFooter.vue";

const {t} = useI18n();
const store = useStore();
const unlisten = ref<(() => void) | null>(null);

onMounted(() => {
  store.init();
  addCloseHandler();
})

onUnmounted(() => {
  if (unlisten.value) {
    unlisten.value();
    unlisten.value = null;
  }
});

const addCloseHandler = async () => {
  unlisten.value = await getCurrentWindow().onCloseRequested(async (event) => {
    if (store.processing) {
      const confirmed = await confirm(t('common.closeAppDialogTitle'), {
        okLabel: t('common.yes'),
        cancelLabel: t('common.no'),
      });
      console.log(confirmed);
      if (!confirmed) {
        event.preventDefault();
        return;
      } else {
        await videoAPI.cancelConversion();
        store.processing = false;
      }
    }
  });
}

</script>

<style>
.app {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  margin: 8px;
}

</style>