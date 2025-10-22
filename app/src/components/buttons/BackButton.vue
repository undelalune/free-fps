<script setup lang="ts">
import {ArrowBack} from "@vicons/tabler";
import {useRouter} from "vue-router";
import {useI18n} from "vue-i18n";

const {t} = useI18n();
const router = useRouter();
const goBack = () => {
  router.back();
}
</script>

<template>
  <n-tooltip placement="bottom" trigger="hover">
    <template #trigger>
      <n-button
          class="back-menu-btn"
          size="small"
          tertiary
          @click="goBack"
      >
        <n-icon size="20">
          <ArrowBack/>
        </n-icon>
      </n-button>
    </template>
    <span>{{ t('common.back') }}</span>
  </n-tooltip>
</template>

<style scoped>
.back-menu-btn {
  position: absolute;
  top: 8px;
  right: 8px;
  padding: 6px;
}
</style>