<script setup lang="ts">

import {SettingsSharp} from "@vicons/ionicons5";
import {paths} from "@/router/paths.ts";
import {useRouter} from "vue-router";
import {useI18n} from "vue-i18n";
import {NButton, NIcon} from "naive-ui";
import {useStore} from "@/stores";

const {t} = useI18n();
const router = useRouter();
const store = useStore();

const toggleShowSettings = () => {
  router.push(paths.settings);
}
</script>

<template>
  <n-tooltip placement="bottom" trigger="hover">
    <template #trigger>
      <n-button
          class="settings-menu-btn"
          size="tiny"
          ghost
          :bordered="false"
          @click="toggleShowSettings"
          :disabled="store.processing"
      >
        <n-icon size="18">
          <SettingsSharp/>
        </n-icon>
      </n-button>
    </template>
    <span>{{ t('common.settings') }}</span>
  </n-tooltip>
</template>

<style scoped>
.settings-menu-btn {
  position: absolute;
  top: 16px;
  right: 28px;
}
</style>