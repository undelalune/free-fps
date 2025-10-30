<script setup lang="ts">
import FFFoundDialog from '@/components/dialogs/FFFoundDialog.vue';
import { useFFToolsDialog } from '@/composables/useFFToolsDialog';

const { showFFPopup } = useFFToolsDialog();
</script>

<template>
  <FFFoundDialog v-model:show="showFFPopup" />
</template>

<style scoped></style>