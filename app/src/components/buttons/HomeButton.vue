<script setup lang="ts">

import {Home} from "@vicons/ionicons5";
import {paths} from "@/router/paths.ts";
import {useRouter} from "vue-router";
import {useI18n} from "vue-i18n";

const {t} = useI18n();
const router = useRouter();
const goHome = () => {
  router.push(paths.main);
}
</script>

<template>
  <n-tooltip placement="bottom" trigger="hover">
    <template #trigger>
      <n-button
          class="home-menu-btn"
          size="tiny"
          ghost
          :bordered="false"
          @click="goHome"
      >
        <n-icon size="18">
          <Home/>
        </n-icon>
      </n-button>
    </template>
    <span>{{ t('common.home') }}</span>
  </n-tooltip>
</template>

<style scoped>
.home-menu-btn {
  position: absolute;
  top: 16px;
  right: 28px;
}
</style>