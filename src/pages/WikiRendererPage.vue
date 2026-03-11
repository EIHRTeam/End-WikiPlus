<template>
  <q-page class="wiki-renderer-page">
    <div v-if="detailLoading" class="row justify-center q-my-xl">
      <q-spinner-dots color="primary" size="40px" />
    </div>

    <div v-else-if="!wikiItem && isDetailRoute" class="upload-section">
      <q-card class="upload-card">
        <q-card-section>
          <div class="text-h6">{{ $t('error') }}</div>
          <div class="text-caption">{{ detailErrorMessage || $t('wiki.noItemInResponse') }}</div>
        </q-card-section>

        <q-card-actions align="right">
          <q-btn flat color="primary" :label="$t('back')" @click="handleBackAction" />
          <q-btn flat color="primary" :label="$t('refresh')" @click="reloadCurrentItem" />
        </q-card-actions>
      </q-card>
    </div>

    <div v-else-if="!wikiItem" class="upload-section">
      <q-card class="upload-card">
        <q-card-section>
          <div class="text-h5">{{ $t('wiki.rendererTitle') }}</div>
          <div class="text-caption">{{ $t('wiki.rendererDesc') }}</div>
        </q-card-section>

        <q-card-section>
          <q-file
            v-model="file"
            :label="$t('wiki.selectFile')"
            accept=".json,application/json"
            :dark="$q.dark.isActive"
            outlined
            @update:model-value="loadFile"
          >
            <template #prepend>
              <q-icon name="attach_file" />
            </template>
          </q-file>

          <div class="text-caption text-grey-7 q-mt-md">
            {{ $t('wiki.manualOnlyHint') }}
          </div>
        </q-card-section>
      </q-card>
    </div>

    <div v-else class="wiki-content">
      <div class="file-tools">
        <q-card class="file-tools-card" flat bordered>
          <q-card-section class="file-tools-section q-pa-sm">
            <div class="row items-center q-gutter-x-sm">
              <q-btn flat dense round color="primary" icon="arrow_back" @click="handleBackAction">
                <q-tooltip>{{ $t('wiki.backToSelection') }}</q-tooltip>
              </q-btn>

              <q-separator vertical inset />

              <div class="text-subtitle2 text-grey-8">
                {{ currentFileName }}
              </div>
            </div>
          </q-card-section>
        </q-card>
      </div>

      <div v-if="itemBrief" class="wiki-hero">
        <div class="hero-card">
          <div v-if="resolvedHeaderBackground" class="hero-backdrop">
            <img :src="resolvedHeaderBackground" :alt="itemBrief.name" />
            <div class="hero-backdrop__scrim"></div>
          </div>

          <div class="hero-body">
            <div
              v-if="resolvedCoverImage"
              class="hero-cover"
              :class="{ 'hero-cover--clickable': canOpenHeroIllustrationViewer }"
            >
              <img
                :src="resolvedCoverImage"
                :alt="itemBrief.name"
                :role="canOpenHeroIllustrationViewer ? 'button' : undefined"
                :tabindex="canOpenHeroIllustrationViewer ? 0 : undefined"
                @click="openHeroIllustrationViewer"
                @keydown.enter.prevent="openHeroIllustrationViewer"
                @keydown.space.prevent="openHeroIllustrationViewer"
              />
            </div>

            <div class="hero-detail">
              <h1 class="hero-title">{{ itemBrief.name || wikiItem?.name }}</h1>

              <div v-if="itemBrief.subTypeList" class="hero-tags">
                <span
                  v-for="subType in itemBrief.subTypeList"
                  :key="subType.subTypeId"
                  class="hero-tag"
                >
                  {{ tagNameMap[subType.value] || subType.value }}
                </span>
              </div>

              <div class="hero-desc">
                <WikiDocument
                  v-if="briefDescriptionDocument"
                  :document="briefDescriptionDocument"
                />
                <template v-else-if="caption">
                  <p v-for="(cap, idx) in caption" :key="idx">{{ cap.text?.text }}</p>
                </template>
              </div>
            </div>
          </div>

          <div v-if="resolvedExtraIllustration" class="hero-figure">
            <img :src="resolvedExtraIllustration" :alt="itemBrief.name" />
          </div>
        </div>
      </div>

      <div class="wiki-body">
        <template v-if="chapterGroup?.length">
          <WikiChapterGroup
            v-for="group in chapterGroup"
            :key="group.title"
            :group="group"
            :widget-common-map="widgetCommonMap"
            :document-map="documentMap"
          />
        </template>

        <template v-else>
          <div class="fallback-docs">
            <section v-for="(doc, docId) in documentMap" :key="docId" class="fallback-section">
              <div class="fallback-title">
                {{ getDocumentTitle(doc, docId) }}
              </div>
              <WikiDocument :document="doc" />
            </section>
          </div>
        </template>
      </div>
    </div>

    <ImageViewer
      :visible="heroImageViewerVisible"
      :src="heroIllustrationViewerSrc"
      :original-url="heroIllustrationOriginalUrl"
      :alt="heroIllustrationViewerAlt"
      @update:visible="heroImageViewerVisible = $event"
    />
  </q-page>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useQuasar } from 'quasar';
import { invoke } from '@tauri-apps/api/core';
import { useRoute, useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';
import WikiDocument from '../components/wiki/WikiDocument.vue';
import ImageViewer from '../components/wiki/ImageViewer.vue';
import WikiChapterGroup from '../components/wiki/layout/WikiChapterGroup.vue';
import { useSklandStore } from '../stores/skland';
import { backOrFallback } from '../utils/navigation';
import { resolveAssetUrl } from '../utils/wiki-loader';
import type {
  ChapterGroup,
  Document,
  TagNode,
  WidgetCommon,
  WikiData,
  WikiItem,
} from '../types/wiki';

const $q = useQuasar();
const route = useRoute();
const router = useRouter();
const sklandStore = useSklandStore();
const { t } = useI18n();

const file = ref<File | null>(null);
const wikiItem = ref<WikiItem | null>(null);
const detailLoading = ref(false);
const detailErrorMessage = ref('');

const routeItemId = computed(() => {
  const raw = route.params.itemId;
  return typeof raw === 'string' ? raw.trim() : '';
});
const isDetailRoute = computed(() => route.name === 'wiki-item');

const currentFileName = computed(
  () => file.value?.name || wikiItem.value?.name || t('wiki.defaultTitle'),
);
const documentMap = computed<Record<string, Document>>(
  () => wikiItem.value?.document?.documentMap || {},
);
const itemBrief = computed(() => wikiItem.value?.brief);
const chapterGroup = computed<ChapterGroup[] | undefined>(
  () => wikiItem.value?.document?.chapterGroup,
);
const widgetCommonMap = computed<Record<string, WidgetCommon>>(
  () => wikiItem.value?.document?.widgetCommonMap || {},
);
const caption = computed(() => wikiItem.value?.caption);

const briefDescriptionDocument = computed(() => {
  const desc = itemBrief.value?.description;
  if (desc && typeof desc === 'object') {
    return desc as Document;
  }
  return null;
});

const resolvedHeaderBackground = ref('');
const resolvedCoverImage = ref('');
const resolvedExtraIllustration = ref('');
const heroImageViewerVisible = ref(false);

watch(
  () => [itemBrief.value?.cover || '', wikiItem.value?.document?.extraInfo?.illustration || ''],
  async ([cover, illustration]) => {
    resolvedCoverImage.value = await resolveAssetUrl(cover ?? '');
    resolvedExtraIllustration.value = await resolveAssetUrl(illustration ?? '');
    resolvedHeaderBackground.value = resolvedExtraIllustration.value || resolvedCoverImage.value;
  },
  { immediate: true },
);

watch(
  () => wikiItem.value?.itemId,
  () => {
    heroImageViewerVisible.value = false;
  },
);

const isCharacterEntry = computed(() => itemBrief.value?.associate?.type === 'char');
const canOpenHeroIllustrationViewer = computed(
  () => isCharacterEntry.value && resolvedExtraIllustration.value.length > 0,
);
const heroIllustrationViewerSrc = computed(() => resolvedExtraIllustration.value);
const heroIllustrationOriginalUrl = computed(
  () => wikiItem.value?.document?.extraInfo?.illustration || '',
);
const heroIllustrationViewerAlt = computed(() => {
  const displayName =
    itemBrief.value?.name || wikiItem.value?.name || t('wiki.defaultCharacterName');
  return t('wiki.illustrationAlt', { name: displayName });
});

const tagNameMap = computed<Record<string, string>>(() => {
  const map: Record<string, string> = {};
  const roots = wikiItem.value?.subType?.filterTagTree || [];

  const walk = (node: TagNode) => {
    map[node.id] = node.name;
    node.children?.forEach(walk);
  };

  roots.forEach(walk);
  return map;
});

function describeError(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

function isInvalidDeviceError(error: unknown) {
  const message = describeError(error);
  const normalized = message.toLowerCase();
  return (
    message.includes('设备信息无效') ||
    normalized.includes('device id') ||
    normalized.includes('device info')
  );
}

async function fetchAndLoadWikiItem(itemId: string) {
  if (!itemId) {
    resetView();
    detailErrorMessage.value = t('wiki.noItemInResponse');
    return;
  }

  if (!window.__TAURI_INTERNALS__) {
    resetView();
    detailErrorMessage.value = t('wiki.manualOnlyHint');
    return;
  }

  detailLoading.value = true;
  detailErrorMessage.value = '';
  sklandStore.initialize();

  try {
    let dId = await sklandStore.ensureDid();
    const requestItem = async (deviceId: string) =>
      await invoke<string>('fetch_wiki_item', {
        itemId,
        dId: deviceId,
        userAgent: sklandStore.userAgent,
      });

    let jsonStr = '';
    try {
      jsonStr = await requestItem(dId);
    } catch (error) {
      if (!isInvalidDeviceError(error)) {
        throw error;
      }
      dId = await sklandStore.refreshDid();
      jsonStr = await requestItem(dId);
    }

    const data: WikiData = JSON.parse(jsonStr);
    if (!data.data?.item) {
      throw new Error(t('wiki.noItemInResponse'));
    }

    file.value = null;
    wikiItem.value = data.data.item;
  } catch (error) {
    console.error('Failed to fetch wiki item:', error);
    resetView();
    detailErrorMessage.value = describeError(error);
    $q.notify({
      type: 'negative',
      message: `${t('error')}: ${detailErrorMessage.value}`,
      timeout: 5000,
    });
  } finally {
    detailLoading.value = false;
  }
}

async function loadFile(newFile: File | null) {
  if (!newFile) {
    resetView();
    return;
  }

  try {
    const text = await newFile.text();
    const data: WikiData = JSON.parse(text);

    if (!data.data?.item) {
      throw new Error(t('wiki.noItemInResponse'));
    }

    file.value = newFile;
    wikiItem.value = data.data.item;
  } catch (error) {
    console.error('Failed to load wiki file:', error);
    resetView();
    $q.notify({
      type: 'negative',
      message: `${t('error')}: ${error instanceof Error ? error.message : String(error)}`,
    });
  }
}

function resetView() {
  file.value = null;
  wikiItem.value = null;
  heroImageViewerVisible.value = false;
  detailErrorMessage.value = '';
}

function openHeroIllustrationViewer() {
  if (!canOpenHeroIllustrationViewer.value) {
    return;
  }
  heroImageViewerVisible.value = true;
}

function handleBackAction() {
  if (isDetailRoute.value) {
    void backOrFallback(router, '/');
    return;
  }

  resetView();
}

function reloadCurrentItem() {
  if (!routeItemId.value) return;
  void fetchAndLoadWikiItem(routeItemId.value);
}

function getDocumentTitle(doc: Document, fallback: string): string {
  for (const blockId of doc.blockIds) {
    const block = doc.blockMap[blockId];
    if (block?.kind === 'text') {
      const kind = block.text?.kind || '';
      if (kind.startsWith('heading') || kind === 'title' || kind === 'subtitle') {
        const text = (block.text.inlineElements || [])
          .filter((el) => el.kind === 'text')
          .map((el) => el.text.text)
          .join('')
          .trim();
        if (text) return text;
      }
    }
  }

  return fallback;
}

watch(
  [() => route.name, () => routeItemId.value],
  ([routeName, itemId]) => {
    if (routeName === 'wiki-item' && itemId) {
      void fetchAndLoadWikiItem(itemId);
      return;
    }

    detailLoading.value = false;
    detailErrorMessage.value = '';
    if (routeName === 'wiki-render' && !file.value) {
      resetView();
    }
  },
  { immediate: true },
);
</script>

<style scoped lang="scss">
.wiki-renderer-page {
  min-height: 100vh;
  height: 100%;
  overflow-y: auto;
  background-color: var(--bg-primary);
}

.upload-section {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 80vh;
  padding: 2rem;
}

.upload-card {
  width: 100%;
  max-width: 560px;
}

.wiki-content {
  background: transparent;
}

.file-tools {
  padding: 1rem 1.5rem 0;
}

.file-tools-card {
  border: 1px solid var(--card-border);
  border-radius: 10px;
  background: var(--card-bg);
}

.file-tools-section {
  padding: 1rem 1.25rem;
}

.wiki-hero {
  margin: 1rem 1.5rem 0;
}

.hero-card {
  position: relative;
  display: flex;
  align-items: flex-end;
  min-height: 380px;
  overflow: hidden;
  border-radius: 16px;
  background: var(--hero-solid, #16213e);

  .body--dark & {
    --hero-solid: #0f1724;
  }
}

.hero-backdrop {
  position: absolute;
  inset: 0;
  pointer-events: none;

  > img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    object-position: top center;
  }
}

.hero-backdrop__scrim {
  position: absolute;
  inset: 0;
  background:
    linear-gradient(
      110deg,
      var(--hero-solid, #16213e) 0%,
      var(--hero-solid, #16213e) 32%,
      color-mix(in srgb, var(--hero-solid, #16213e) 82%, transparent) 48%,
      color-mix(in srgb, var(--hero-solid, #16213e) 40%, transparent) 64%,
      color-mix(in srgb, var(--hero-solid, #16213e) 15%, transparent) 80%,
      transparent 100%
    ),
    linear-gradient(
      to top,
      color-mix(in srgb, var(--hero-solid, #16213e) 70%, transparent) 0%,
      transparent 50%
    );
}

.hero-body {
  position: relative;
  z-index: 2;
  display: flex;
  align-items: flex-end;
  gap: 1.5rem;
  width: 100%;
  max-width: 800px;
  padding: 2.5rem;
}

.hero-cover {
  width: 128px;
  height: 128px;
  flex-shrink: 0;
  overflow: hidden;
  border: 3px solid rgba(255, 255, 255, 0.18);
  border-radius: 14px;
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.35);

  img {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
}

.hero-cover--clickable {
  cursor: pointer;

  img {
    transition:
      transform 0.2s ease,
      box-shadow 0.2s ease;
  }

  &:hover img {
    transform: scale(1.03);
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.35);
  }
}

.hero-detail {
  flex: 1;
  min-width: 0;
  color: #fff;
}

.hero-title {
  margin: 0 0 0.5rem;
  font-size: 2.6rem;
  font-weight: 800;
  line-height: 1.12;
  letter-spacing: -0.015em;
  text-shadow: 0 1px 4px rgba(0, 0, 0, 0.45);
}

.hero-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.45rem;
  margin-bottom: 0.65rem;
}

.hero-tag {
  display: inline-flex;
  align-items: center;
  padding: 0.15rem 0.6rem;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 4px;
  background: rgba(255, 255, 255, 0.14);
  color: rgba(255, 255, 255, 0.92);
  font-size: 0.74rem;
  font-weight: 600;
  letter-spacing: 0.02em;
}

.hero-desc {
  max-width: 560px;
  color: rgba(255, 255, 255, 0.82);
  font-size: 0.9rem;
  line-height: 1.65;

  :deep(.wiki-document) {
    color: rgba(255, 255, 255, 0.82) !important;

    p {
      margin: 0.2rem 0;
    }
  }

  p {
    margin: 0.2rem 0;
  }
}

.hero-figure {
  position: absolute;
  right: 0;
  bottom: 0;
  z-index: 1;
  display: flex;
  align-items: flex-end;
  justify-content: flex-end;
  height: 110%;
  pointer-events: none;

  img {
    height: 100%;
    max-width: 520px;
    object-fit: contain;
    object-position: bottom right;
    filter: drop-shadow(0 4px 16px rgba(0, 0, 0, 0.35));
  }
}

.wiki-body {
  position: relative;
  z-index: 1;
  max-width: 1200px;
  margin: 2rem auto 0;
  padding: 2rem;
  background: var(--card-bg);
}

.fallback-docs {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.fallback-section {
  padding-bottom: 1.5rem;
  border-bottom: 1px solid #eee;
}

.fallback-section:last-child {
  border-bottom: none;
}

.fallback-title {
  margin-bottom: 1rem;
  color: #333;
  font-size: 1.1rem;
  font-weight: 600;

  .body--dark & {
    color: #e0e0e0;
  }
}

@media (max-width: 960px) {
  .hero-figure {
    opacity: 0.3;
    mask-image: linear-gradient(to left, black 0%, transparent 80%);
  }
}

@media (max-width: 640px) {
  .upload-section {
    min-height: auto;
    padding: 1rem 0.75rem;
  }

  .file-tools,
  .wiki-hero {
    padding: 0;
    margin: 0.75rem 0.75rem 0;
  }

  .hero-card {
    min-height: 260px;
  }

  .hero-body {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
    padding: 1.5rem;
  }

  .hero-cover {
    width: 88px;
    height: 88px;
    border-radius: 10px;
  }

  .hero-title {
    font-size: 1.6rem;
  }

  .hero-figure {
    display: none;
  }

  .wiki-body {
    margin-top: 1rem;
    padding: 1rem 0.75rem 1.5rem;
  }
}
</style>
