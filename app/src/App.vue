<!--  Free FPS - Video Frame Rate Converter-->
<!--  Copyright (C) 2025 undelalune-->
<!-- -->
<!--  This program is free software: you can redistribute it and/or modify-->
<!--  it under the terms of the GNU General Public License as published by-->
<!--  the Free Software Foundation, either version 3 of the License, or-->
<!--  (at your option) any later version.-->
<!-- -->
<!--  This program is distributed in the hope that it will be useful,-->
<!--  but WITHOUT ANY WARRANTY; without even the implied warranty of-->
<!--  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the-->
<!--  GNU General Public License for more details.-->
<!-- -->
<!--  You should have received a copy of the GNU General Public License-->
<!--  along with this program.  If not, see <https://www.gnu.org/licenses/>.-->

<template>
  <n-config-provider :theme-overrides="store.theme">
    <n-global-style/>
    <div id="app" class="app">
      <n-message-provider>
        <MainView/>
      </n-message-provider>
      <n-modal-provider>
        <HelpDialog/>
      </n-modal-provider>
    </div>
    <AppFooter/>
  </n-config-provider>
</template>

<script setup lang="ts">
import {useStore} from "@/stores";
import {onMounted, onBeforeUnmount} from "vue";
import HelpDialog from "@/components/dialogs/HelpDialog.vue";
import {getCurrentWindow} from "@tauri-apps/api/window";
import {confirm} from '@tauri-apps/plugin-dialog';
import {tauriAPI} from "@/api/tauri.ts";
import {useI18n} from "vue-i18n";
import AppFooter from "@/components/AppFooter.vue";
import MainView from "@/views/MainView.vue";

const {t} = useI18n();
const store = useStore();
const unlisten = ref<(() => void) | null>(null);

onMounted(() => {
  store.init();
  addCloseHandler();
})

onBeforeUnmount(async () => {
  try {
    if (unlisten.value) {
      unlisten.value();
    }
  } catch (error) {
    console.warn('Failed to unlisten:', error);
  } finally {
    unlisten.value = null;
  }
});

const addCloseHandler = async () => {
  try {
    unlisten.value = await getCurrentWindow().onCloseRequested(async (event) => {
      if (store.processing) {
        const confirmed = await confirm(t('common.closeAppDialogTitle'), {
          okLabel: t('common.yes'),
          cancelLabel: t('common.no'),
        });
        if (!confirmed) {
          event.preventDefault();
          return;
        } else {
          await tauriAPI.cancelConversion();
          store.processing = false;
        }
      }
    });
  } catch (error) {
    console.error('Failed to add close handler:', error);
  }
}

</script>

<style>
.app {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  margin: 8px;
}

</style>