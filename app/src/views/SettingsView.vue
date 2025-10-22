<script setup lang="ts">
import LogoHeader from "@/components/LogoHeader.vue";
import HomeButton from "@/components/buttons/HomeButton.vue";
import {CircleX, File, InfoCircle, Moon, Sun} from "@vicons/tabler";
import {useI18n} from "vue-i18n";
import {useStore} from "@/stores";
import {languages} from "@/i18n";
import {platform} from "@tauri-apps/plugin-os";
import {open} from '@tauri-apps/plugin-dialog';
import ResetSettingsButton from "@/components/buttons/ResetSettingsButton.vue";

const {t} = useI18n();
const store = useStore();
const selectFilePath = async (isFfmpeg: boolean = true) => {
  try {
    const currentPlatform = platform();

    const selected = await open({
      directory: false,
      multiple: false,
      filters: [
        {name: t('settingsView.executables'), extensions: currentPlatform === 'windows' ? ['exe'] : ['']},
      ],
    });

    if (selected) {
      if (isFfmpeg) store.ffmpegPath = selected;
      else store.ffprobePath = selected;
    }
  } catch (e) {
    console.error('File selection was cancelled or failed:', e);
  }
};

</script>

<template>
  <n-flex align="center" vertical>
    <LogoHeader/>
    <HomeButton/>

    <div class="settings-wrapper">
      <n-form-item v-if="!store.ffmpegUseInstalled"
                   :label="t('settingsView.pathToFfmpeg')"
                   required
                   :show-feedback="false">
        <n-input :value="store.ffmpegPath"
                 :placeholder="t('settingsView.ffmpegPlaceholder')"
                 readonly clearable
                 @click="selectFilePath()">
          <template #clear-icon>
            <n-icon :component="CircleX" @click.stop="store.ffmpegPath=''"/>
          </template>
          <template #suffix>
            <n-icon :component="File"/>
          </template>
        </n-input>
      </n-form-item>
      <n-checkbox v-if="store.ffmpegInstalledVersion"
                  class="use-installed-cb"
                  v-model:checked="store.ffmpegUseInstalled">
        {{ t('settingsView.useInstalled') }} ffmpeg
      </n-checkbox>

      <n-form-item v-if="!store.ffprobeUseInstalled"
                   class="settings-form-item"
                   :label="t('settingsView.pathToFfprobe')"
                   :show-feedback="false">
        <n-input :value="store.ffprobePath"
                 :placeholder="t('settingsView.ffprobePlaceholder')"
                 readonly clearable
                 @click="selectFilePath(false)">
          <template #clear-icon>
            <n-icon :component="CircleX" @click.stop="store.ffprobePath=''"/>
          </template>
          <template #suffix>
            <n-icon :component="File"/>
          </template>
        </n-input>
      </n-form-item>
      <n-checkbox v-if="store.ffprobeInstalledVersion" class="use-installed-cb"
                  v-model:checked="store.ffprobeUseInstalled">
        {{ t('settingsView.useInstalled') }} ffprobe
      </n-checkbox>

      <n-form-item class="settings-form-item" :label="t('settingsView.chooseLanguage')">
        <n-select :value="store.locale"
                  @update:value="store.setLocale($event)"
                  :options="languages"
        />
      </n-form-item>

      <n-flex align="center" justify="space-between">
        <n-form-item :label="store.isDarkTheme ? t('settingsView.darkTheme') : t('settingsView.lightTheme')">
          <n-switch :value="store.isDarkTheme" @update:value="store.switchTheme()">
            <template #icon>
              <n-icon>
                <Moon v-if="store.isDarkTheme"/>
                <Sun v-else/>
              </n-icon>
            </template>
          </n-switch>
        </n-form-item>
        <ResetSettingsButton/>
      </n-flex>

      <div class="help-block">
        <n-text>
          {{ t('settingsView.helpText') }}
        </n-text>
        <n-tooltip placement="top">
          <template #trigger>
            <n-button quaternary circle type="primary" @click="store.showHelp = true" size="small"
                      @mouseenter="store.heartIsBeating = true" @mouseleave="store.heartIsBeating = false">
              <template #icon>
                <n-icon>
                  <InfoCircle/>
                </n-icon>
              </template>
            </n-button>
          </template>
          <span>{{ t('settingsView.openHelp') }}</span>
        </n-tooltip>
      </div>
    </div>
  </n-flex>

</template>

<style scoped>
.settings-wrapper {
  display: flex;
  flex-direction: column;
  width: 90%;
  margin-top: 36px;
}

.settings-form-item {
  margin-top: 20px;
}

.use-installed-cb {
  margin: 12px 0 0 0;
}

.help-block {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 4px;
}
</style>