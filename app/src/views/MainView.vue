<script setup lang="ts">
import {onMounted, onUnmounted, ref, watch} from 'vue';
import {useStore} from '@/stores';
import {useI18n} from 'vue-i18n';
import {useMessage} from 'naive-ui';
import {useVideoConversion} from '@/composables/useVideoConversion';

import SettingsButton from '@/components/buttons/SettingsButton.vue';
import HelpButton from '@/components/buttons/HelpButton.vue';
import LogoHeader from '@/components/LogoHeader.vue';
import Setup from '@/components/main/Setup.vue';
import Processing from '@/components/main/Processing.vue';
import {RotateClockwise2} from "@vicons/tabler";
import TitledHeader from "@/components/TitledHeader.vue";

const store = useStore();
const {t} = useI18n();
const message = useMessage();
const setupState = ref(true);

const getTitle = () => {
  return `${t('common.conversion')} • ${store.useCustomFps ? store.customFps : store.targetFps} fps`;
};
const {
  isFfmpegConfigured,
  scanFolder,
  rescanFolder,
  performConversion,
  cancelConversion,
  setupProgressListener,
  cleanup,
} = useVideoConversion();

const onNext = () => {
  if (!isFfmpegConfigured.value) {
    message.warning(t('mainView.setup.ffmpegNotSet'), {duration: 5000, closable: true});
    return;
  }
  rescanFolder();
  setupState.value = false;
};

watch(() => store.inputFolder, scanFolder);

onMounted(setupProgressListener);
onUnmounted(cleanup);
</script>

<template>
  <div class="main-view">
    <LogoHeader v-if="setupState"/>
    <TitledHeader v-else :title="getTitle()">
      <RotateClockwise2/>
    </TitledHeader>
    <HelpButton :edged="!setupState"/>
    <SettingsButton v-if="setupState"/>

    <div class="main-view-container">
      <Setup v-if="setupState" @next="onNext"/>
      <Processing
          v-else
          @convert="performConversion"
          @stop="cancelConversion"
          @back="setupState = true"
          @rescan="rescanFolder"
      />
    </div>
  </div>
</template>

<style scoped>
.main-view {
  display: flex;
  flex-flow: column;
  justify-content: start;
  align-items: center;
}

.main-view-container {
  display: flex;
  flex-direction: column;
  width: 90%;
  margin-top: 46px;
}
</style>