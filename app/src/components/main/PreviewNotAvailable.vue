<script setup lang="ts">

import {AlertTriangle} from "@vicons/tabler";
import {useThemeVars} from 'naive-ui'
import {useI18n} from "vue-i18n";

const {t} = useI18n();
const themeVars = useThemeVars();
</script>

<template>

  <div class="preview-not-available">
    <n-icon size="24" :color="themeVars.errorColor">
      <AlertTriangle/>
    </n-icon>
    <div class="preview-not-available--message">
      {{t('mainView.processing.previewNotAvailable')}}
    </div>

  </div>

</template>

<style scoped>
.preview-not-available {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 90%;
  border-radius: 8px;
  border: 1px dashed v-bind('themeVars.textColor2');
  padding: 2px;
}

.preview-not-available--message {
  margin: 2px;
  font-size: 12px;
  max-width: 180px;
  text-align: center;
  font-weight: bold;
}
</style>