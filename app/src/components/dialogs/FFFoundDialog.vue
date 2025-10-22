<script setup lang="ts">
import {useI18n} from "vue-i18n";
import {useStore} from "@/stores";
import {useRouter} from "vue-router";
import {paths} from "@/router/paths.ts";


const {t} = useI18n();
const store = useStore();
const router = useRouter();

const emit = defineEmits<{
  (e: 'update:show', value: Boolean): void;
}>();

const onOkClick = () => {
  emit('update:show', false);
}
const onSettingsClick = () => {
  emit('update:show', false);
  router.push(paths.settings);
}
</script>

<template>
  <n-modal style="width: 90%;" transform-origin="center" :mask-closable="false">
    <n-card
        :title="store.ffmpegInstalledVersion || store.ffprobeInstalledVersion ? t('fFFoundDialog.foundFfToolsTitle') : t('fFFoundDialog.noFfToolsTitle')"
        header-style="text-align: center;"
        :bordered="true"
        role="dialog"
        aria-modal="true"
    >
      <n-flex v-if="store.ffmpegInstalledVersion || store.ffprobeInstalledVersion">
        <n-text>{{ t('fFFoundDialog.foundFfToolsDesc') }}</n-text>
        <div class="ff-version-msg">
          <span class="ff-version-msg__title">ffmpeg:</span>
          <n-text class="ff-version-msg__desc">
            {{ store.ffmpegInstalledVersion ? store.ffmpegInstalledVersion : t('fFFoundDialog.noVersion') }}
          </n-text>
        </div>
        <div class="ff-version-msg">
          <span class="ff-version-msg__title">ffprobe:</span>
          <n-text class="ff-version-msg__desc">
            {{ store.ffprobeInstalledVersion ? store.ffprobeInstalledVersion : t('fFFoundDialog.noVersion') }}
          </n-text>
        </div>
        <n-flex align="flex-start" vertical style="margin-top: 16px; gap: 8px;">
          <n-checkbox v-if="store.ffmpegInstalledVersion" v-model:checked="store.ffmpegUseInstalled">
            {{ t('fFFoundDialog.useInstalled') }} ffmpeg
          </n-checkbox>
          <n-checkbox v-if="store.ffprobeInstalledVersion" v-model:checked="store.ffprobeUseInstalled">
            {{ t('fFFoundDialog.useInstalled') }} ffprobe
          </n-checkbox>
        </n-flex>
      </n-flex>
      <template #footer>
        <n-flex align="center" justify="center">
          <n-button :type="store.ffmpegInstalledVersion || store.ffprobeInstalledVersion ? 'default' : 'primary'" @click="onSettingsClick">{{ t('fFFoundDialog.goToSettings') }}</n-button>
          <n-button v-if="store.ffmpegInstalledVersion || store.ffprobeInstalledVersion" type="primary" @click="onOkClick">{{ t('common.ok') }}</n-button>
        </n-flex>
      </template>
    </n-card>

  </n-modal>
</template>

<style scoped>
.ff-version-msg {
  margin-top: 8px;
  display: flex;
  flex-flow: row nowrap;
  gap: 16px;
  width: 100%;
}

.ff-version-msg__title {
  flex-shrink: 0;
  font-weight: bold;
}

.ff-version-msg__desc {
  flex-grow: 1;
  word-break: break-all;
}
</style>