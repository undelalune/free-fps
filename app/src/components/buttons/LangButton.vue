<script setup lang="ts">

import {World} from "@vicons/tabler";
import {useI18n} from "vue-i18n";
import {useStore} from "@/stores";
import {languages} from "@/i18n";

const {t} = useI18n();
const store = useStore();
</script>

<template>
  <n-dropdown trigger="click"
              :value="store.locale"
              key-field="value"
              @select="store.setLocale($event)"
              :options="languages"
  >
    <n-tooltip placement="bottom" trigger="hover" :delay="500">
      <template #trigger>
        <n-button
            color="#888" style="padding: 0;"
            size="tiny"
            ghost
            :bordered="false"
        >
          <n-icon size="18">
            <World/>
          </n-icon>
        </n-button>
      </template>
      <span>{{ t('common.language') }}</span>
    </n-tooltip>
  </n-dropdown>
</template>
