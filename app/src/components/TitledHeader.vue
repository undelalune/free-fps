<script setup lang="ts">
import { useThemeVars } from 'naive-ui';
defineProps<{ title: string }>();

const themeVars = useThemeVars();
</script>

<template>
  <div class="logo-header">
    <div class="logo-header__wrapper">
      <n-text class="logo-header__title">{{ title }}</n-text>
    </div>

  </div>
</template>

<style scoped>
.logo-header {
  width: 100%;
}

.logo-header__wrapper {
  display: flex;
  align-items: center;
  gap: 8px;
  padding-top: 8px;
  padding-left: 22px;
}

.logo-header__title {
  font-size: 18px;
  line-height: 22px;
  z-index: 2;
}
</style>
