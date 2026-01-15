<script setup lang="ts">
import {useI18n} from "vue-i18n";
import CloseButton from "@/components/buttons/CloseButton.vue";
import {computed, nextTick, onBeforeUnmount, ref, watch} from "vue";
import MarkdownIt from "markdown-it";
import DOMPurify from "dompurify";
import {useStore} from "@/stores";
import {openUrl} from "@tauri-apps/plugin-opener";
import {LicenseType} from "@/types";
import {tauriAPI} from "@/api/tauri";

const store = useStore();
const {t, locale} = useI18n();

const mdContainerEl = ref<HTMLElement | null>(null);
let detach: (() => void) | null = null;

watch(
    () => store.showHelp,
    async (show) => {
      if (show) {
        await nextTick();
        attachOnce();
      } else {
        detach?.();
      }
    },
    {immediate: true}
);

const rawMdByLocale = import.meta.glob("/src/md/*.{md,MD}", {
  eager: true,
  query: "?raw",
  import: "default",
}) as Record<string, string>;

const md = new MarkdownIt({html: true, linkify: true, typographer: true});

const pickMdForLocale = (loc: string) => {
  const candidates = [`/src/md/${loc}.MD`, `/src/md/${loc}.md`, `/src/md/en.MD`, `/src/md/en.md`];
  for (const key of candidates) if (key in rawMdByLocale) return rawMdByLocale[key];
  return Object.values(rawMdByLocale)[0] ?? "";
}

const renderedHtml = computed(() => {
  const raw = pickMdForLocale(locale.value);
  const currentYear = new Date().getFullYear().toString();
  const processedRaw = raw.replace(/\{\{YEAR}}/g, currentYear);
  const html = md.render(processedRaw);
  return DOMPurify.sanitize(html);
});

const handler = async (e: MouseEvent) => {
  const target = e.target as HTMLElement | null;
  const a = target?.closest?.("a") as HTMLAnchorElement | null;
  if (!a) return;

  const href = a.getAttribute("href") || "";

  // external https links - unchanged
  if (/^(https:)/i.test(href)) {
    e.preventDefault();
    try {
      await openUrl(href);
    } catch (err) {
      console.error("Failed to open external link:", err);
    }
    return;
  }

  const lt = resolveLicenseType(href);
  if (lt) {
    e.preventDefault();
    try {
      await tauriAPI.openLicense(lt);
    } catch (err) {
      console.error("Failed to open bundled license:", err);
    }
    return;
  }
};

function resolveLicenseType(href: string): LicenseType | null {
  const licenseMap: Record<string, LicenseType> = {
    FFmpegNotice: LicenseType.FFmpegNotice,
    FFmpegLicense: LicenseType.FFmpegLicense,
    FreeFPSLicense: LicenseType.FreeFPSLicense,
  };

  for (const key of Object.keys(licenseMap)) {
    if (href.includes(key)) return licenseMap[key as keyof typeof licenseMap];
  }
  return null;
}

const attachOnce = () => {
  if (detach) return;
  const el = mdContainerEl.value;
  if (!el) return;
  el.addEventListener("click", handler, {passive: false});
  detach = () => {
    el.removeEventListener("click", handler);
    detach = null;
  };
}

const getVersion = computed(() => {
  return `version: ${import.meta.env.VITE_APP_VERSION || "unknown"}`;
});
onBeforeUnmount(() => detach?.());
</script>

<template>
  <n-modal v-model:show="store.showHelp"
           class="help-dialog"
           transform-origin="center"
           :mask-closable="false"
           :closable="true">
    <n-card
        :title="t('common.helpTitle')"
        :bordered="true"
        aria-modal="true"
        content-class="help-card"
    >
      <template #header-extra>
        <CloseButton/>
      </template>


      <n-scrollbar class="help-scroll-area" contentStyle="padding-right: 12px;">
        <div ref="mdContainerEl" v-html="renderedHtml"></div>
        <div style="width:100%; text-align: center;">{{ getVersion }}</div>
      </n-scrollbar>

    </n-card>

  </n-modal>
</template>

<style>
.help-dialog {
  width: 98%;
  height: 670px;
}

.help-card {
  display: block;
  position: relative;
}

.help-scroll-area {
  width: 100%;
  max-height: 580px;
}

a {
  color: deepskyblue !important;
}

hr {
  height: 1px !important;
  background-color: #384167 !important;
  border: none !important;
}
</style>