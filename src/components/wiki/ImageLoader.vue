<template>
  <div class="wiki-image-loader" :class="[variantClass, { 'is-loading': isLoading, 'has-error': hasError }]">
    <img v-if="!hasError" :src="displayUrl" :alt="alt" :style="imageStyle" class="wiki-image"
      :class="{ 'is-clickable': enableViewer }" referrerpolicy="no-referrer" @load="onLoad" @error="onError"
      @click="onImageClick" />
    <div v-else class="image-error">
      <q-icon name="broken_image" size="48px" color="grey-5" />
      <div class="error-text">{{ t('imageViewer.loadFailed') }}</div>
      <div class="error-url">{{ originalUrl }}</div>
      <a :href="originalUrl" target="_blank" class="open-link">{{ t('imageViewer.openInNewTab') }}</a>
    </div>
    <div v-if="isLoading" class="image-loading">
      <q-spinner-dots color="primary" size="40px" />
    </div>

    <!-- 图片查看器 -->
    <ImageViewer v-if="enableViewer" :visible="viewerVisible" :src="displayUrl" :original-url="url" :alt="alt ?? ''"
      @update:visible="viewerVisible = $event" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { resolveAssetUrl } from '../../utils/wiki-loader';
import ImageViewer from './ImageViewer.vue';

const props = defineProps<{
  url: string;
  alt?: string;
  maxWidth?: number;
  variant?: 'inline' | 'block';
  enableViewer?: boolean;
}>();

const isLoading = ref(true);
const hasError = ref(false);
const resolvedUrl = ref('');
const viewerVisible = ref(false);
const { t } = useI18n();

const originalUrl = computed(() => props.url ?? '');

const displayUrl = computed(() => {
  return resolvedUrl.value || originalUrl.value;
});

const imageStyle = computed(() => {
  const style: Record<string, string> = {};
  if (props.maxWidth) {
    // 使用 min() 函数确保图片最大宽度不超过容器宽度的 100%
    // 这样在移动端或窄屏设备上，即使指定了较大的 maxWidth，图片也不会溢出容器
    style.maxWidth = `min(${props.maxWidth}px, 100%)`;
  }
  return style;
});

const variantClass = computed(() => {
  return props.variant === 'inline' ? 'variant-inline' : 'variant-block';
});

function onLoad() {
  isLoading.value = false;
  hasError.value = false;
}

function onError() {
  isLoading.value = false;
  hasError.value = true;
}

function onImageClick() {
  if (!props.enableViewer || hasError.value || isLoading.value) return;
  viewerVisible.value = true;
}

// 当 URL 变化时重置状态
watch(
  () => props.url,
  async (value) => {
    isLoading.value = true;
    hasError.value = false;
    resolvedUrl.value = await resolveAssetUrl(value || '');
  },
  { immediate: true },
);
</script>

<style scoped lang="scss">
.wiki-image-loader {
  position: relative;
  display: inline-block;
  text-align: center;
  width: 100%;

  &.is-loading {
    min-height: 60px;
  }

  &.has-error {
    padding: 2rem;
    // background: #fafafa;
    border: 1px dashed #e0e0e0;
    border-radius: 8px;
  }
}

.variant-block {
  margin: 1.5em 0;
}

.variant-inline {
  margin: 0;
  width: auto;
}

.wiki-image {
  max-width: 100%;
  height: auto;
  border-radius: 4px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);

  &.is-clickable {
    cursor: pointer;
    transition: transform 0.2s ease, box-shadow 0.2s ease;

    &:hover {
      transform: scale(1.02);
      box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
    }

    &:active {
      transform: scale(0.98);
    }
  }
}

.image-loading {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  // background: rgba(255, 255, 255, 0.9);
  padding: 1rem;
  border-radius: 8px;
}

.image-error {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  color: #999;

  .error-text {
    font-size: 0.9rem;
    color: #666;
  }

  .error-url {
    font-size: 0.75rem;
    color: #999;
    word-break: break-all;
    max-width: 100%;
    font-family: monospace;
  }

  .open-link {
    color: #1976d2;
    text-decoration: none;
    font-size: 0.85rem;

    &:hover {
      text-decoration: underline;
    }
  }
}
</style>
