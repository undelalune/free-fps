<script setup lang="ts">
import {useStore} from "@/stores";
import {useI18n} from "vue-i18n";

const store = useStore();
const {t} = useI18n();
</script>

<template>
  <n-tooltip placement="top">
    <template #trigger>
      <n-button
          class="reset-settings"
          size="small"
          quaternary
          @click="store.resetSettings()"
      >{{ t('settingsView.resetSettings') }}
      </n-button>
    </template>
    <span>{{ t('settingsView.resetSettingsDesc') }}</span>
  </n-tooltip>
</template>

<style scoped>

.reset-settings {
  padding: 6px;
  opacity: 0.5;
}

.reset-settings:hover {
  opacity: 1;
}


</style>