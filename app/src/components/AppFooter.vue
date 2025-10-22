<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, ref, watch } from 'vue';
import FlyingBubble from './FlyingBubble.vue';
import { openUrl } from '@tauri-apps/plugin-opener';
import { useI18n } from 'vue-i18n';
import { useStore } from '@/stores';
import { useThemeVars } from 'naive-ui';

const REDIRECT_URL = 'https://www.buymeacoffee.com/undelalune';
const CLICK_RESET_MS = 2000;
const BUBBLE_LIFETIME_MS = 1050;
const HEART_BEAT_TIMES = 2;
const HEART_BEAT_DURATION_MS = 300;

const store = useStore();
const themeVars = useThemeVars();
const { t } = useI18n();

const words = computed<string[]>(() =>
    String(t('common.ty') ?? '')
        .split('$')
        .map(s => s.trim())
        .filter(Boolean),
);

const isBeating = ref(false);
let heartBeatTimeoutId: number | null = null;

function triggerHeartBeat(times = HEART_BEAT_TIMES, durationMs = HEART_BEAT_DURATION_MS) {
  if (heartBeatTimeoutId != null) {
    clearTimeout(heartBeatTimeoutId);
    heartBeatTimeoutId = null;
  }
  isBeating.value = false;
  nextTick(() => {
    isBeating.value = true;
  });
  heartBeatTimeoutId = window.setTimeout(() => {
    isBeating.value = false;
    heartBeatTimeoutId = null;
  }, times * durationMs + 50);
}

watch(
    () => store.heartIsBeating,
    v => {
      if (v) triggerHeartBeat();
    },
);

type Bubble = { id: number; message: string; offsetX: number };
const bubbles = ref<Bubble[]>([]);
let nextBubbleId = 1;
const bubbleTimeouts = new Map<number, number>();

function removeBubble(id: number) {
  const i = bubbles.value.findIndex(b => b.id === id);
  if (i !== -1) bubbles.value.splice(i, 1);
}

function addBubble(message: string) {
  const id = nextBubbleId++;
  const offsetX = Math.round((Math.random() - 0.5) * 20);
  bubbles.value.push({ id, message, offsetX });

  const tid = window.setTimeout(() => {
    removeBubble(id);
    bubbleTimeouts.delete(id);
  }, BUBBLE_LIFETIME_MS);
  bubbleTimeouts.set(id, tid);
}

const wordIndex = ref(0);
const lastClickAt = ref<number | null>(null);

async function handleHeartClick() {
  const now = Date.now();
  if (lastClickAt.value == null || now - lastClickAt.value > CLICK_RESET_MS) {
    wordIndex.value = 0;
  }
  lastClickAt.value = now;

  if (wordIndex.value >= words.value.length) {
    try {
      await openUrl(REDIRECT_URL);
    } catch {
    }
    return;
  }

  addBubble(words.value[wordIndex.value]);
  wordIndex.value++;
}

onBeforeUnmount(() => {
  if (heartBeatTimeoutId != null) clearTimeout(heartBeatTimeoutId);
  bubbleTimeouts.forEach(tid => clearTimeout(tid));
  bubbleTimeouts.clear();
});
</script>

<template>
  <footer class="app-footer">
    <div class="footer-inner">
      <span
          class="heart"
          role="button"
          tabindex="0"
          aria-label="Send love"
          @click="handleHeartClick"
          @keydown.enter="handleHeartClick"
          @keydown.space.prevent="handleHeartClick"
      >
        <span class="heart-glyph" :class="{ 'is-beating': isBeating }">❤️</span>
        <span class="bubble-host">
          <FlyingBubble
              v-for="b in bubbles"
              :key="b.id"
              :message="b.message"
              :offset-x="b.offsetX"
          />
        </span>
      </span>

      <span> © 2025 <a href="https://github.com/undelalune/free-fps" target="_blank">undelalune</a></span>
    </div>
  </footer>
</template>

<style scoped>
.app-footer {
  position: absolute;
  bottom: 0;
  width: 100%;
  padding: 0.5rem 0;
  text-align: center;
  color: v-bind('themeVars.tableColorStriped') !important;
  opacity: 0.5;
  font-size: 12px;
  transition: opacity 0.3s;
  user-select: none;
}

.app-footer a {
  color: v-bind('themeVars.tableColorStriped') !important;
  text-decoration: none;
}

.app-footer a:hover {
  text-decoration: underline;
}

.footer-inner {
  position: relative;
  display: inline-block;
}

.app-footer:hover,
.app-footer:hover a {
  opacity: 1;
  color: v-bind('themeVars.invertedColor') !important;
}

.heart {
  cursor: pointer;
  margin-right: 0.3rem;
  position: relative;
  display: inline-flex;
}

.heart-glyph {
  display: inline-block;
  transform-origin: center;
}

.heart-glyph.is-beating {
  animation: heart-beat 0.3s ease-in-out 2;
}

@keyframes heart-beat {
  0%,
  100% {
    transform: scale(1);
  }
  30% {
    transform: scale(1.3);
  }
  60% {
    transform: scale(0.9);
  }
}

.bubble-host {
  position: absolute;
  bottom: 0;
  left: 50%;
  transform: translateX(-50%);
  width: 0;
  height: 0;
  pointer-events: none;
}
</style>
