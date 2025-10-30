<script setup lang="ts">
import {X} from "@vicons/tabler";
import {useStore} from "@/stores";

const store = useStore();
const doClose = () => {
  store.showHelp = false;
}
</script>

<template>
  <n-button
      class="close-btn"
      size="small"
      quaternary circle
      @click="doClose"
  >
    <n-icon size="20">
      <X/>
    </n-icon>
  </n-button>
</template>

<style scoped>
.close-btn {
  position: absolute;
  top: 6px;
  right: 6px;
}
</style>