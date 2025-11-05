<script setup lang="ts">

import {Help} from "@vicons/tabler";
import {useI18n} from "vue-i18n";
import {useStore} from "@/stores";

const {t} = useI18n();
const store = useStore();

const toggleShowHelp = () => {
  store.showHelp = true;
}
</script>

<template>
  <n-tooltip placement="bottom" trigger="hover" :delay="500">
    <template #trigger>
      <n-button
          color="#888" style="padding: 0;"
          size="tiny"
          ghost
          :bordered="false"
          @click="toggleShowHelp"
          @mouseenter="store.heartIsBeating = true"
          @mouseleave="store.heartIsBeating = false"
      >
        <n-icon size="18">
          <Help/>
        </n-icon>
      </n-button>
    </template>
    <span>{{ t('common.help') }}</span>
  </n-tooltip>
</template>
