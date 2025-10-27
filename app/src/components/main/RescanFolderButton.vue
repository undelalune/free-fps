<script setup lang="ts">

import {Refresh} from "@vicons/tabler";
import {useStore} from "@/stores";
import {useI18n} from "vue-i18n";

const {t} = useI18n();

defineEmits<{
  (e: 'rescan'): void
}>();

const store = useStore();

defineProps<{
  labeled?: boolean
}>();

const isDisabled = computed(() => {
  return store.folderScanning || store.processing;
})
</script>

<template>
  <n-button v-if="labeled"
            class="rescan-folder-btn"
            :disabled="isDisabled"
            icon-placement="right"
            tertiary
            @click="$emit('rescan')">
    {{ t('mainView.processing.rescanFolder') }}
  </n-button>
  <n-tooltip v-else placement="bottom" trigger="hover" delay="500">
    <template #trigger>
      <n-button class="rescan-folder-btn" size="small" tertiary :disabled="isDisabled"
                @click="$emit('rescan')">
        <template #icon>
          <n-icon>
            <Refresh/>
          </n-icon>
        </template>
      </n-button>
    </template>
    <span>{{ t('mainView.processing.rescanFolder') }}</span>
  </n-tooltip>

</template>

<style scoped>
</style>