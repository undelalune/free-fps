<script setup lang="ts">
import {useStore} from "@/stores";
import VideoItem from "@/components/main/VideoItem.vue";
import {useI18n} from "vue-i18n";
import RescanFolderButton from "@/components/main/RescanFolderButton.vue";
import {useThemeVars} from "naive-ui";
import {ConversionStatus} from "@/types";

defineEmits<{
  back: []
  convert: []
  stop: []
  rescan: []
}>();

const store = useStore();
const {t} = useI18n();
const themeVars = useThemeVars();
const selectAll = ref(false);

const selectedFiles = computed(() => store.videoFiles.filter(v => v.convert));
const allSelected = computed(() => store.videoFiles.length > 0 && store.videoFiles.every(v => v.convert));

const selectedInfo = computed(() => {
  const count = selectedFiles.value.length;
  return count ? t('mainView.processing.videoSelected', {num: count}) : t('mainView.processing.noSelection');
});

const statusInfo = computed(() => {
  const total = selectedFiles.value.length;
  if (!total) return '';

  const completed = selectedFiles.value.filter(v => v.progress === 100).length;
  return completed === total
      ? t('mainView.processing.allProcessed')
      : t('mainView.processing.processingStatus', {completed, total});
});

const canConvert = computed(() =>
    !store.folderScanning && selectedFiles.value.some(v => v.status !== ConversionStatus.Success)
);

const toggleSelectAll = (checked: boolean) => {
  selectAll.value = checked ? !allSelected.value : false;
  store.videoFiles.forEach(v => v.convert = selectAll.value);
};

</script>

<template>
  <n-flex>
    <div v-if="!store.videoFiles.length" class="processing-no-files-found">
      <n-text>{{ t('mainView.processing.noFilesFound') }}</n-text>
      <RescanFolderButton @rescan="$emit('rescan')" labeled/>
    </div>
    <n-spin v-else :show="store.folderScanning" style="width:100%; height: 100%;">
      <n-flex>
        <div class="processing-header-container">
          <n-checkbox :checked="allSelected"
                      @update:checked="toggleSelectAll"
                      class="select-all-checkbox"
                      :label="t('mainView.processing.selectAll')"
                      :disabled="store.processing"/>
          <n-text>{{ selectedInfo }}</n-text>
        </div>

        <n-scrollbar class="video-scroll-container">
          <VideoItem v-for="v in store.videoFiles" :key="v.name" :videoItem="v" :processing="store.processing"/>
        </n-scrollbar>

        <div class="under-video-list">
          <RescanFolderButton @rescan="$emit('rescan')"/>
          <n-text>{{ statusInfo }}</n-text>
        </div>
      </n-flex>
    </n-spin>

    <n-button
        class="prev-btn"
        :disabled="store.processing"
        @click="$emit('back')"
    >{{ t('common.back') }}
    </n-button>

    <n-button
        class="action-btn"
        type="primary"
        :disabled="store.processing ? false : !canConvert"
        @click="store.processing ? $emit('stop') : $emit('convert')"
    >
      {{ t(store.processing ? 'mainView.processing.stop' : 'mainView.processing.convert') }}
    </n-button>

  </n-flex>
</template>

<style>
.processing-no-files-found {
  display: flex;
  flex-flow: column;
  align-items: center;
  gap: 8px;
  width: 100%;
  text-align: center;
  margin-top: 20px;
}

.processing-header-container {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding-bottom: 4px;
  border-bottom: 1px solid v-bind('themeVars.tableColorHover');
}

.select-all-checkbox {
  margin-left: 10px;
}

.video-scroll-container {
  max-height: 430px;
  width: 100%;
}

.under-video-list {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  padding: 0;
}

.n-scrollbar > .n-scrollbar-rail.n-scrollbar-rail--vertical--right {
  right: 0 !important;
}

.prev-btn {
  position: absolute;
  bottom: 40px;
  left: 30px;
  padding: 16px;
}

.action-btn {
  position: absolute;
  bottom: 40px;
  right: 30px;
  padding: 16px;
}
</style>