<script setup lang="ts">
import {ConversionStatus, VideoFile} from "@/types";
import {AlertCircle, CircleCheck} from '@vicons/tabler';
import {useThemeVars} from 'naive-ui'
import {useI18n} from "vue-i18n";


const themeVars = useThemeVars();
const {t} = useI18n();
const props = defineProps<{ videoItem: VideoFile; processing: boolean }>();

const formatSize = (bytes: number = 0): string => {
  const sizes = ['b', 'kb', 'mb', 'gb'];
  if (bytes === 0) return t('mainView.processing.b', {size: 0});
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  const size = Math.round((bytes / Math.pow(1024, i)) * 100) / 100;
  return t(`mainView.processing.${sizes[i]}`, {size: size});
};

const showProgress = computed(() => {
  return props.videoItem?.status === ConversionStatus.Processing;
});

const showError = computed(() => {
  return props.videoItem?.status === ConversionStatus.Error;
})

const showSuccess = computed(() => {
  return props.videoItem?.status === ConversionStatus.Success;
})


</script>

<template>
  <div class="video-item"
       :class="{ 'video-item-deselected': !videoItem.convert, 'video-item-disabled': processing, 'video-item-deselected-disabled': !videoItem.convert && processing }"
       @click="videoItem.convert=!videoItem.convert">
    <div class="col-checkbox">
      <n-checkbox :checked="videoItem.convert" :disabled="processing"/>
    </div>

    <div class="video-info">
      <div class="video-title" :title="videoItem.name">
        {{ videoItem.name }}
      </div>
      <div class="video-details">
        <span class="video-size">{{ formatSize(videoItem.size) }}</span>
      </div>
    </div>

    <div class="col-progress">
      <n-progress
          v-if="showProgress"
          class="progress-parsing"
          type="circle"
          :percentage="videoItem.progress"
          :offset-degree="180"
      />
      <n-icon v-else-if="showSuccess" class="progress-status" size="20"
              :color="themeVars.successColor">
        <CircleCheck/>
      </n-icon>
      <n-icon v-else-if="showError" class="progress-status" size="20"
              :color="themeVars.errorColor">
        <AlertCircle/>
      </n-icon>
    </div>
  </div>
</template>

<style scoped>
.video-item {
  display: flex;
  min-width: 0;
  height: 42px;
  align-items: center;
  gap: 12px;
  padding: 8px;
  border-bottom: 1px solid v-bind('themeVars.tableColorHover');
  background-color: v-bind('themeVars.tableColor');
  border-radius: 6px;
  margin: 0 0 2px 0;
  cursor: pointer;
  transition: background-color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.video-item:hover {
  background-color: v-bind('themeVars.tableHeaderColor');
  transition: background-color 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.video-item-deselected {
  opacity: 0.7;
}

.video-item-disabled {
  pointer-events: none;
  opacity: 0.9;
}

.video-item-deselected-disabled {
  opacity: 0.3;
  pointer-events: none;
}

.col-checkbox {
  flex: 0 0 24px;
  display: flex;
  justify-content: center;
}

.video-info {
  flex: 1 1 auto;
  min-width: 0; /* critical for WebView2/Chromium */
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.video-title {
  display: block; /* or inline-block */
  width: 100%; /* fill available space */
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-weight: bold;
}

.video-details {
  font-size: 12px;
}

.col-progress {
  flex: 0 0 36px;
  display: flex;
  justify-content: flex-end;
  margin-right: 2px;
}

.progress-parsing {
  width: 36px;
  height: 36px;
}

.progress-parsing * {
  max-height: 36px;
  max-width: 36px;
  font-size: 11px !important;
}

.progress-status {
  margin-right: 8px;
}

</style>
