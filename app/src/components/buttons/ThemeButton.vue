<script setup lang="ts">

import {useI18n} from "vue-i18n";
import {NButton, NIcon} from "naive-ui";
import {useStore} from "@/stores";
import {Moon, Sun} from "@vicons/tabler";

const {t} = useI18n();
const store = useStore();

</script>

<template>
  <n-tooltip placement="bottom" trigger="hover" :delay="500">
    <template #trigger>
      <n-button
          color="#888" style="padding: 0;"
          size="tiny"
          ghost
          :bordered="false"
          @click="store.switchTheme()"
          :disabled="store.processing"
      >
        <n-icon size="18">
          <Sun v-if="store.isDarkTheme"/>
          <Moon v-else/>
        </n-icon>

      </n-button>
    </template>
    <span>{{ store.isDarkTheme ? t('common.lightTheme') : t('common.darkTheme') }}</span>
  </n-tooltip>
</template>
