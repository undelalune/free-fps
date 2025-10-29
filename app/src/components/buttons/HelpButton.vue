<script setup lang="ts">

import {Help} from "@vicons/tabler";
import {useI18n} from "vue-i18n";
import {useStore} from "@/stores";

const {t} = useI18n();
const store = useStore();


defineProps<{
  edged?: boolean
}>();


const toggleShowHelp = () => {
  store.showHelp = true;
}
</script>

<template>
  <n-tooltip placement="bottom" trigger="hover" delay="500">
    <template #trigger>
      <n-button
          class="help-menu-btn"
          size="tiny"
          ghost
          :bordered="false"
          :style="{ right: edged ? '28px' : '52px' }"
          @click="toggleShowHelp"
          @mouseenter="store.heartIsBeating = true"
          @mouseleave="store.heartIsBeating = false"
          :disabled="store.processing"
      >
        <n-icon size="18">
          <Help/>
        </n-icon>
      </n-button>
    </template>
    <span>{{ t('common.help') }}</span>
  </n-tooltip>
</template>

<style scoped>
.help-menu-btn {
  position: absolute;
  top: 16px;
  right: 52px;
  color: #888;
}
</style>