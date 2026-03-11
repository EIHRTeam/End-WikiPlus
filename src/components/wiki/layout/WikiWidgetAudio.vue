<template>
  <div class="wiki-widget-audio">
    <div class="audio-list">
      <div v-for="(audio, index) in resolvedAudioList" :key="index" class="audio-item">
        <div class="audio-info">
          <div class="audio-title">{{ audio.title }}</div>
          <div class="audio-profile">{{ audio.profile }}</div>
          <div v-if="showAudioSaveButton" class="audio-actions">
            <button class="audio-save-btn" type="button" :disabled="isSaving(audio.resourceUrl)"
              @click="saveAudio(audio)">
              {{ isSaving(audio.resourceUrl) ? t('loading') : t('wiki.saveAudio') }}
            </button>
          </div>
        </div>
        <div class="audio-player">
          <audio controls controlslist="nodownload" :src="audio.blobUrl" referrerpolicy="no-referrer">
            {{ t('wiki.audioUnsupported') }}
          </audio>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch, onBeforeUnmount } from 'vue';
import { useQuasar } from 'quasar';
import { useI18n } from 'vue-i18n';
import type { WidgetCommon } from '../../../types/wiki';
import { resolveAssetUrl } from '../../../utils/wiki-loader';
import {
  describeInvokeError,
  isIOSTauriRuntime,
  isMobileTauriRuntime,
  saveMediaFromUrl,
  saveMediaFromBlob,
} from 'src/utils/android-media';

const props = defineProps<{
  widgetCommon: WidgetCommon;
}>();

interface AudioItem {
  title: string;
  profile: string;
  resourceUrl: string;
}

interface ResolvedAudioItem extends AudioItem {
  blobUrl: string;
  blob: Blob | null;
  mimeType: string;
  fileName: string;
}

const $q = useQuasar();
const { t } = useI18n();
const httpUrlPattern = /^https?:\/\//i;
const isMobileDevice = computed(() => $q.platform.is.mobile === true || isMobileTauriRuntime());
const showAudioSaveButton = computed(() => !isMobileDevice.value);
const mimeToExt: Record<string, string> = {
  'audio/mpeg': 'mp3',
  'audio/mp3': 'mp3',
  'audio/mp4': 'm4a',
  'audio/x-m4a': 'm4a',
  'audio/wav': 'wav',
  'audio/x-wav': 'wav',
  'audio/ogg': 'ogg',
  'audio/aac': 'aac',
  'audio/flac': 'flac',
  'audio/opus': 'opus',
  'audio/webm': 'webm',
};
const extToMime: Record<string, string> = {
  mp3: 'audio/mpeg',
  m4a: 'audio/mp4',
  wav: 'audio/wav',
  ogg: 'audio/ogg',
  aac: 'audio/aac',
  flac: 'audio/flac',
  opus: 'audio/opus',
  webm: 'audio/webm',
};

const audioList = computed<AudioItem[]>(() => {
  const defaultTab = props.widgetCommon.tabDataMap?.default;
  if (defaultTab && Array.isArray(defaultTab.audioList)) {
    return defaultTab.audioList as AudioItem[];
  }
  return [];
});

const resolvedAudioList = ref<ResolvedAudioItem[]>([]);
const savingMap = ref<Record<string, boolean>>({});

// Track blob URLs for cleanup
const blobUrls: string[] = [];

function revokeBlobUrls() {
  blobUrls.forEach((url) => URL.revokeObjectURL(url));
  blobUrls.length = 0;
}

function sanitizeFileName(name: string): string {
  return name.replace(/[\\/:*?"<>|]/g, '_');
}

function getFilenameFromUrl(url: string): string {
  try {
    const pathname = new URL(url).pathname;
    const segments = pathname.split('/');
    const last = segments[segments.length - 1];
    if (last && last.includes('.')) return decodeURIComponent(last);
  } catch {
    // ignore
  }
  return 'audio.mp3';
}

function guessAudioMimeType(filename: string): string {
  const ext = filename.split('.').pop()?.toLowerCase() || '';
  return extToMime[ext] || 'audio/mpeg';
}

function ensureAudioFilename(name: string, mimeType: string): string {
  const safeName = sanitizeFileName(name);
  if (safeName.includes('.')) return safeName;
  const ext = mimeToExt[mimeType] || 'mp3';
  return `${safeName}.${ext}`;
}

function triggerBrowserDownload(url: string, filename: string) {
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
}

/**
 * Desktop-only fallback: write to Downloads via plugin-fs.
 * NOT used on mobile — on Android, BaseDirectory.Download resolves to app-private directory,
 * not the public Downloads folder. Mobile must always use the native MediaStore plugin.
 */
async function saveBlobToTauriDownloads(blob: Blob, fileName: string) {
  const { writeFile, BaseDirectory } = await import('@tauri-apps/plugin-fs');
  const bytes = new Uint8Array(await blob.arrayBuffer());
  await writeFile(fileName, bytes, { baseDir: BaseDirectory.Download });
}

function isSaving(resourceUrl: string): boolean {
  return savingMap.value[resourceUrl] === true;
}

function getAudioSaveCaption(fileName: string): string {
  return isIOSTauriRuntime()
    ? t('wiki.audioSaveNotifyBodyIos', { name: fileName })
    : t('wiki.audioSaveNotifyBody', { name: fileName });
}

async function fetchAudioBlob(item: ResolvedAudioItem): Promise<Blob | null> {
  if (item.blob) return item.blob;

  try {
    const blobResp = await fetch(item.blobUrl);
    if (blobResp.ok) return await blobResp.blob();
  } catch {
    // continue
  }

  try {
    const response = await fetch(item.resourceUrl);
    if (response.ok) return await response.blob();
  } catch {
    // continue
  }

  if (!!window.__TAURI_INTERNALS__ && httpUrlPattern.test(item.resourceUrl)) {
    try {
      const { fetch: tauriFetch } = await import('@tauri-apps/plugin-http');
      const response = await tauriFetch(item.resourceUrl);
      if (response.ok) return await response.blob();
    } catch {
      // continue
    }
  }

  return null;
}

async function saveAudio(item: ResolvedAudioItem) {
  if (isMobileDevice.value) return;

  const key = item.resourceUrl;
  if (isSaving(key)) return;
  savingMap.value[key] = true;

  try {
    const mimeType = item.mimeType || guessAudioMimeType(getFilenameFromUrl(item.resourceUrl));
    const fileName = ensureAudioFilename(item.fileName || getFilenameFromUrl(item.resourceUrl), mimeType);
    const isTauri = !!window.__TAURI_INTERNALS__;

    if (isTauri) {
      // Primary path: Rust backend downloads from CDN — no blob involved.
      const isHttpUrl = httpUrlPattern.test(item.resourceUrl);
      if (isHttpUrl) {
        try {
          await saveMediaFromUrl(item.resourceUrl, fileName, mimeType, 'downloads');
          $q.notify({
            type: 'positive',
            message: t('wiki.audioSaveSuccess'),
            caption: getAudioSaveCaption(fileName),
            timeout: 2200,
          });
          return;
        } catch {
          // URL download failed, fall through to blob fallback
        }
      }

      // Fallback: fetch blob in JS, send base64 to Rust backend
      const blob = await fetchAudioBlob(item);
      if (!blob) {
        $q.notify({ type: 'negative', message: t('wiki.audioSaveFailed') });
        return;
      }
      const blobMime = blob.type || mimeType;
      const blobFileName = ensureAudioFilename(item.fileName || getFilenameFromUrl(item.resourceUrl), blobMime);
      try {
        await saveMediaFromBlob(blob, blobFileName, blobMime, 'downloads');
      } catch (blobError) {
        // Desktop-only fallback via plugin-fs.
        // NEVER use on mobile — BaseDirectory.Download resolves to app-private directory.
        if (!isMobileTauriRuntime()) {
          try {
            await saveBlobToTauriDownloads(blob, blobFileName);
          } catch {
            throw blobError;
          }
        } else {
          throw blobError;
        }
      }
    } else {
      // Web browser: fetch blob and trigger download
      const blob = await fetchAudioBlob(item);
      if (!blob) {
        $q.notify({ type: 'negative', message: t('wiki.audioSaveFailed') });
        return;
      }
      const url = URL.createObjectURL(blob);
      triggerBrowserDownload(url, fileName);
      URL.revokeObjectURL(url);
    }

    $q.notify({
      type: 'positive',
      message: t('wiki.audioSaveSuccess'),
      caption: getAudioSaveCaption(fileName),
      timeout: 2200,
    });
  } catch (error: unknown) {
    const detail = describeInvokeError(error);
    const needsPermission = detail.includes('Storage permission requested');
    if (needsPermission) {
      $q.notify({
        type: 'negative',
        message: t('wiki.storagePermissionRequested'),
        timeout: 2600,
      });
    } else {
      $q.notify({
        type: 'negative',
        message: t('wiki.audioSaveFailed'),
        caption: detail,
        timeout: 2600,
      });
    }
  } finally {
    delete savingMap.value[key];
  }
}

watch(
  audioList,
  async (list) => {
    revokeBlobUrls();
    savingMap.value = {};
    resolvedAudioList.value = await Promise.all(
      list.map(async (item) => {
        const resolvedUrl = await resolveAssetUrl(item.resourceUrl);
        // Fetch audio as blob so the native <audio> download button works (same-origin blob URL)
        let blobUrl = resolvedUrl;
        let blob: Blob | null = null;
        try {
          const resp = await fetch(resolvedUrl);
          if (resp.ok) {
            blob = await resp.blob();
            blobUrl = URL.createObjectURL(blob);
            blobUrls.push(blobUrl);
          }
        } catch {
          // Fallback to direct URL if fetch fails
        }
        const rawFileName = getFilenameFromUrl(resolvedUrl);
        const mimeType = blob?.type || guessAudioMimeType(rawFileName);
        const fileName = ensureAudioFilename(rawFileName, mimeType);
        return { ...item, resourceUrl: resolvedUrl, blobUrl, blob, mimeType, fileName };
      }),
    );
  },
  { immediate: true },
);

onBeforeUnmount(() => {
  revokeBlobUrls();
  savingMap.value = {};
});
</script>

<style scoped lang="scss">
.wiki-widget-audio {
  width: 100%;
}

.audio-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.audio-item {
  display: flex;
  flex-direction: column;
  padding: 1rem;
  border: 1px solid rgba(0, 0, 0, 0.12);
  border-radius: 8px;
  background: rgba(0, 0, 0, 0.02);
  gap: 0.5rem;
}

.body--dark .audio-item {
  border-color: rgba(255, 255, 255, 0.12);
  background: rgba(255, 255, 255, 0.05);
}

.audio-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  flex: 1;
  min-width: 0;
}

.audio-title {
  font-weight: 600;
  font-size: 1rem;
}

.audio-profile {
  font-size: 0.9rem;
  opacity: 0.8;
  font-style: italic;
}

.audio-actions {
  margin-top: 0.5rem;
}

.audio-save-btn {
  border: 1px solid rgba(0, 0, 0, 0.18);
  border-radius: 8px;
  background: rgba(0, 0, 0, 0.05);
  color: var(--text-primary, rgba(0, 0, 0, 0.87));
  font-size: 0.85rem;
  font-weight: 600;
  line-height: 1;
  padding: 0.5rem 0.75rem;
  cursor: pointer;
  transition: background-color 0.15s ease, border-color 0.15s ease;

  &:hover {
    background: rgba(0, 0, 0, 0.08);
  }

  &:active {
    background: rgba(0, 0, 0, 0.12);
  }

  &:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
}

.body--dark .audio-save-btn {
  border-color: rgba(255, 255, 255, 0.18);
  background: rgba(255, 255, 255, 0.08);
  color: rgba(255, 255, 255, 0.9);

  &:hover {
    background: rgba(255, 255, 255, 0.12);
  }

  &:active {
    background: rgba(255, 255, 255, 0.18);
  }
}

.audio-player {
  width: 100%;
  margin-top: 0.5rem;

  audio {
    width: 100%;
    height: 40px;
  }
}
</style>
