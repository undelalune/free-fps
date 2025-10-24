<script setup lang="ts">
defineProps<{ title: string }>();
</script>

<template>
  <n-flex
      class="logo-header"
      justify="center"
      align="center"
      :size="12"
  >
    <n-text class="logo-header__title">{{ title }}</n-text>
    <n-icon :size="24">
      <slot/>
    </n-icon>
  </n-flex>
</template>

<style scoped>
.logo-header {
  position: relative;
  width: 100%;
  margin-top: 12px;
  opacity: 0.9;
}

.logo-header__title {
  font-size: 24px;
  font-weight: 600;
  line-height: 36px;
}
</style>
