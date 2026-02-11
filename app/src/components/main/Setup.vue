<script setup lang="ts">
import {computed} from 'vue';
import {useStore} from '@/stores';
import {useI18n} from 'vue-i18n';
import {CircleX, Folder, InfoCircle} from '@vicons/tabler';
import {open} from "@tauri-apps/plugin-dialog";

const store = useStore();
const {t} = useI18n();

const fpsList = computed(() => [
  {label: t('mainView.setup.customFpsLabel'), value: 0},
  {label: '12', value: 12},
  {label: '16', value: 16},
  {label: '18', value: 18},
  {label: '23.976', value: 23.976},
  {label: '24', value: 24},
  {label: '25', value: 25},
  {label: '29.97', value: 29.97},
  {label: '30', value: 30},
  {label: '47.952', value: 47.952},
  {label: '48', value: 48},
  {label: '50', value: 50},
  {label: '59.94', value: 59.94},
  {label: '60', value: 60},
  {label: '72', value: 72},
  {label: '90', value: 90},
  {label: '95.904', value: 95.904},
  {label: '96', value: 96},
  {label: '100', value: 100},
  {label: '119.88', value: 119.88},
  {label: '120', value: 120}
]);

const inputFeedback = computed(() => {
  if (store.folderScanning || !store.inputFolder) return '';
  return store.videoFiles.length ? t('mainView.setup.videosFound', {count: store.videoFiles.length}) : t('mainView.setup.noVideosFound');
});
const selectFolderPath = async (isInputFolder: boolean = true) => {
  try {
    const selected = await open({directory: true, multiple: false});
    if (selected) {
      if (isInputFolder) store.inputFolder = selected as string;
      else store.outputFolder = selected as string;
    }
  } catch (e) {
    console.error('Folder selection was cancelled or failed:', e);
  }
};

const outputPlaceholder = computed(() => {
  const fps = store.useCustomFps ? store.customFps : store.targetFps;
  const folder = `converted_videos_${fps}fps`;
  const separator = store.inputFolder?.includes('\\') ? '\\' : '/';
  return store.inputFolder ? `${store.inputFolder}${separator}${folder}` : folder;
});


const selectFps = (value: number) => {
  store.useCustomFps = value === 0;
  store.customFps = 30;
};
const formatVideoTooltip = (value: number): string => {
  if (value <= 18) return t('mainView.setup.losslessQuality', {value});
  else if (value <= 28) return t('mainView.setup.highQuality', {value});
  else if (value <= 40) return t('mainView.setup.mediumQuality', {value});
  else return t('mainView.setup.lowQuality', {value});
}
const formatCPUTooltip = (value: number): string => `${value}%`;
</script>

<template>
  <n-form :model="store" label-placement="top" size="medium">
    <!-- Input Folder -->
    <n-form-item :feedback="inputFeedback" feedback-style="text-align: right;" required>
      <template #label>
        <span class="label-with-help">
          {{ t('mainView.setup.inputFolder') }}
          <n-tooltip placement="top" style="max-width:300px" :delay="500">
            <template #trigger>
              <n-icon size="16"><InfoCircle/></n-icon>
            </template>
            {{ t('mainView.setup.inputFolderInfo') }}
          </n-tooltip>
        </span>
      </template>
      <n-input v-model:value="store.inputFolder" @click="selectFolderPath(true)" readonly placeholder=""
               :loading="store.folderScanning">
        <template #suffix>
          <n-icon v-if="!store.folderScanning" :component="Folder"/>
        </template>
      </n-input>
    </n-form-item>

    <!-- Output Folder -->
    <n-form-item>
      <template #label>
        <span class="label-with-help">
          {{ t('mainView.setup.outputFolder') }}
          <n-tooltip placement="top" style="max-width:240px" :delay="500">
            <template #trigger>
              <n-icon size="16"><InfoCircle/></n-icon>
            </template>
            {{ t('mainView.setup.outputFolderInfo') }}
          </n-tooltip>
        </span>
      </template>
      <n-input v-model:value="store.outputFolder" @click="selectFolderPath(false)" readonly clearable :placeholder="outputPlaceholder">
        <template #clear-icon>
          <n-icon :component="CircleX" @click.stop="store.outputFolder=''"/>
        </template>
        <template #suffix>
          <n-icon :component="Folder"/>
        </template>
      </n-input>
    </n-form-item>

    <!-- FPS (grouped) -->
    <div class="horizontal-flex-group">
      <n-flex>
        <n-form-item>
          <template #label>
            <span class="label-with-help">
              {{ t('mainView.setup.targetFps') }}
              <n-tooltip placement="top" style="max-width:280px" :delay="500">
                <template #trigger>
                  <n-icon size="16"><InfoCircle/></n-icon>
                </template>
                {{ t('mainView.setup.fpsInfo') }}
              </n-tooltip>
            </span>
          </template>
          <n-select class="target-fps-select" v-model:value="store.targetFps" :options="fpsList"
                    @update:value="selectFps($event)"
                    :consistent-menu-width="false"/>
        </n-form-item>
      </n-flex>
      <n-flex>
        <n-form-item v-if="store.targetFps === 0" :label="t('mainView.setup.customFps')">
          <n-input-number class="custom-fps-input"
                          v-model:value="store.customFps"
                          :min="1"
                          :max="200"
                          :precision="3"
                          :step="1"
                          placeholder=""
          />
        </n-form-item>
      </n-flex>
    </div>


    <div class="horizontal-flex-group">
      <n-flex vertical>
        <!-- Audio (grouped) -->
        <div class="non-form-el">
          <n-checkbox v-model:checked="store.keepAudio" :label="t('mainView.setup.keepAudio')"/>
          <n-tooltip placement="top" style="max-width:200px" :delay="500">
            <template #trigger>
              <n-icon size="16">
                <InfoCircle/>
              </n-icon>
            </template>
            {{ t('mainView.setup.audioQualityInfo') }}
          </n-tooltip>
        </div>
        <n-form-item v-if="store.keepAudio" :show-feedback="false">
          <template #label>
            <span class="label-with-help">
              {{ t('mainView.setup.audioBitrate') }}
            </span>
          </template>
          <n-input-number class="audio-bitrate-input"
                          v-model:value="store.audioBitrate"
                          :min="32"
                          :max="320"
                          :step="16"
                          placeholder=""
          >
            <template #suffix>{{ t('mainView.setup.kbps') }}</template>
          </n-input-number>
        </n-form-item>
      </n-flex>
      <n-flex vertical>
        <!-- GPU Acceleration -->
        <div class="non-form-el">
          <n-checkbox
            v-model:checked="store.useGpu"
            :disabled="!store.gpuInfo || store.gpuInfo.gpu_type === 'None'"
            :label="t('mainView.setup.useGpu')"
          />
          <n-tooltip placement="top" style="max-width:240px" :delay="500">
            <template #trigger>
              <n-icon size="16"><InfoCircle/></n-icon>
            </template>
            {{ t('mainView.setup.gpuInfo') }}
          </n-tooltip>
        </div>
        <div v-if="store.gpuDetecting" class="gpu-status">
          <n-spin size="small" />
          <n-text depth="3" style="font-size: 12px; margin-left: 8px;">
            {{ t('mainView.setup.detectingGpu') }}
          </n-text>
        </div>
        <div v-else-if="store.gpuInfo && store.gpuInfo.gpu_type !== 'None'" class="gpu-status">
          <n-text depth="3" class="gpu-model-name">
            {{ store.gpuInfo.model_name }}
          </n-text>
        </div>
        <div v-else-if="store.gpuInfo" class="gpu-status">
          <n-text depth="3" style="font-size: 12px;">
            {{ t('mainView.setup.noGpuDetected') }}
          </n-text>
        </div>
      </n-flex>
    </div>

    <!-- Video quality and CPU limit row -->
    <div class="horizontal-flex-group bottom-settings-row">
      <n-flex vertical>
        <!-- Video quality (grouped) -->
        <div class="non-form-el">
          <n-checkbox v-model:checked="store.useCustomVideoQuality" style="max-width: 180px;"
                      :label="t('mainView.setup.customVideoQuality')"/>
          <n-tooltip placement="top" style="max-width:200px" :delay="500">
            <template #trigger>
              <n-icon size="16">
                <InfoCircle/>
              </n-icon>
            </template>
            {{ t('mainView.setup.customVideoQualityInfo') }}
          </n-tooltip>
        </div>
        <n-form-item v-if="store.useCustomVideoQuality" :label="t('mainView.setup.setVideoQuality')"
                     :show-feedback="false">
          <template #label>
            <span class="label-with-help">
              {{ t('mainView.setup.setVideoQuality') }}
              <n-tooltip placement="top" style="max-width: 260px;" :delay="500">
                <template #trigger>
                  <n-icon size="16"><InfoCircle/></n-icon>
                </template>
                {{ t('mainView.setup.setVideoQualityInfo') }}
              </n-tooltip>
            </span>
          </template>
          <n-slider v-model:value="store.videoQuality" :min="0" :max="51" :step="1"
                    :format-tooltip="formatVideoTooltip"/>
        </n-form-item>
      </n-flex>
      <n-flex vertical>
        <!-- CPU limit -->
        <n-form-item :show-feedback="false">
          <template #label>
            <span class="label-with-help">
              {{ t('mainView.setup.cpuLimit') }}
              <n-tooltip placement="top" style="max-width:240px" :delay="500">
                <template #trigger>
                  <n-icon size="16"><InfoCircle/></n-icon>
                </template>
                {{ t('mainView.setup.cpuLimitInfo') }}
              </n-tooltip>
            </span>
          </template>
          <n-slider class="cpu-slider"
                    v-model:value="store.cpuLimit"
                    :min="5"
                    :max="100"
                    :step="1"
                    :format-tooltip="formatCPUTooltip"
          />
        </n-form-item>
      </n-flex>
    </div>

    <!-- Next -->
    <n-button class="next-btn" type="primary" :disabled="!store.inputFolder || store.folderScanning"
              @click="$emit('next')">
      {{ t('common.next') }}
    </n-button>
  </n-form>
</template>

<style scoped>
.label-with-help {
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.target-fps-select, .audio-bitrate-input {
  width: 160px;
}

.custom-fps-input {
  width: 112px;
}

.cpu-slider {
  width: 160px;
}

.non-form-el {
  display: flex;
  align-items: center;
  margin-bottom: 12px;
}

.horizontal-flex-group {
  display: flex;
  align-items: flex-start;
  gap: 24px;
}

.horizontal-flex-group > *:first-child {
  width: 180px;
}

.gpu-status {
  display: flex;
  align-items: center;
  padding: 4px 0;
}

.gpu-model-name {
  font-size: 12px;
  max-width: 180px;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
  line-height: 1.4;
}

.bottom-settings-row {
  margin-top: 20px;
}

.next-btn {
  position: absolute;
  bottom: 30px;
  right: 30px;
  padding: 16px;
}
</style>
