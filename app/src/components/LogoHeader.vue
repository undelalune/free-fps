<script setup lang="ts">
const getVersion = ('v'+import.meta.env.VITE_APP_VERSION) || 'v0.0.0'
</script>

<template>
  <n-flex
      class="logo-header"
      justify="center"
      align="flex-end"
      :size="12"
  >
    <n-text class="logo-header__title">Free</n-text>
    <n-image
        src="logo.png"
        width="36"
        height="36"
        :preview-disabled="true"
        :title="getVersion"
        alt="logo"
    />
  </n-flex>
</template>

<style scoped>
.logo-header {
  position: relative;
  width: 100%;
  margin-top: 12px;
}

.logo-header__title {
  font-size: 36px;
  font-weight: 500;
  line-height: 36px;
}
</style>
