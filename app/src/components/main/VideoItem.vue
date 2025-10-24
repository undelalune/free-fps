<script setup lang="ts">
import {ConversionStatus, VideoFile} from "@/types";
import {AlertCircle, CircleCheck, Eye, AlertTriangle} from '@vicons/tabler';
import {useThemeVars} from 'naive-ui'
import {useI18n} from "vue-i18n";
import {videoAPI} from "@/api/tauri.ts";
import PreviewNotAvailable from "@/components/main/PreviewNotAvailable.vue";


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

const thumbnail = ref('');
const thumbnailError = ref(false);
const showPreview = ref(false);
const getThumbnail = () => {
  showPreview.value = true;
  if (thumbnail.value) return;
  videoAPI.getVideoThumbnail(props.videoItem.path).then((thumbPath) => {
    thumbnail.value = thumbPath;
    thumbnailError.value = thumbPath === null;
  }).catch((e) => {
    thumbnailError.value = true;
    console.error('Failed to get thumbnail for video:', props.videoItem.path, e);
  });
};

</script>

<template>
  <div class="video-item"
       :class="{
          'video-item-deselected': !videoItem.convert,
          'video-item-disabled': processing,
          'video-item-deselected-disabled': !videoItem.convert && processing,
          'video-item--preview': showPreview
        }"
       @click="videoItem.convert=!videoItem.convert">
    <div class="col-checkbox">
      <n-checkbox :checked="videoItem.convert" :disabled="processing"/>
    </div>

    <div v-if="showPreview"
         class="video-preview"
         @mouseleave="showPreview = false">
      <PreviewNotAvailable v-if="thumbnailError"/>
      <n-image v-else-if="thumbnail" :src="thumbnail" width="200" style="border-radius: 8px;" />
      <n-spin v-else size="tiny" content-class="preview-spinner" />
    </div>

    <div class="video-info">
      <div class="video-title" :class="{'video-title--preview': showPreview}" :title="videoItem.name">
        {{ videoItem.name }}
      </div>
      <div v-if="!showPreview" class="video-details">
        <n-button size="tiny" ghost quaternary  @click.stop="getThumbnail">
          <template #icon>
            <n-icon size="16"><Eye /></n-icon>
          </template>
        </n-button>
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
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.video-item--preview {
  height: 120px;
}

.video-item:hover {
  background-color: v-bind('themeVars.tableHeaderColor');
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
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

.video-preview {
  position: relative;
  flex: 0 0 200px;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: space-around;
  overflow: hidden;
  border-radius: 4px;
}
.preview-spinner {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
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

.video-title--preview {
  display: -webkit-box;
  -webkit-line-clamp: 5;
  -webkit-box-orient: vertical;
  word-wrap: anywhere;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: normal;
  animation: preview-appear 0.2s cubic-bezier(0.4, 0, 0.2, 1) 0.15s both;
}

@keyframes preview-appear {
  from { opacity: 0; transform: translateY(-6px); }
  to   { opacity: 1; transform: translateY(0); }
}

.video-details {
  font-size: 12px;
  display: flex;
  align-items: center;
  gap: 6px;
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
