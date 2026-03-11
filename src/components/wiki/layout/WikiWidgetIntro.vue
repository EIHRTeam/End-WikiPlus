<template>
  <div v-if="intro" class="widget-intro">
    <div v-if="intro.imgUrl" class="intro-media">
      <ImageLoader :url="intro.imgUrl" :alt="intro.name" variant="block" />
    </div>
    <div class="intro-content">
      <div class="intro-title">
        <span class="intro-name">{{ intro.name }}</span>
        <span v-if="intro.type" class="intro-type">{{ intro.type }}</span>
      </div>

      <div v-if="introDescriptionDocument" class="intro-description">
        <WikiDocument :document="introDescriptionDocument" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { WidgetIntro, Document } from '../../../types/wiki';
import WikiDocument from '../WikiDocument.vue';
import ImageLoader from '../ImageLoader.vue';

const props = defineProps<{
  intro: WidgetIntro;
  documentMap: Record<string, Document>;
}>();

const introDescriptionDocument = computed(() => {
  if (!props.intro?.description) return null;
  return props.documentMap[props.intro.description] || null;
});
</script>

<style scoped lang="scss">
.widget-intro {
  display: flex;
  gap: 1.5rem;
  padding: 1rem;
  border-bottom: 1px dashed var(--card-border);
  margin-bottom: 1.5rem;
  align-items: flex-start;
  transition: all 0.3s ease;

  // 移动端适配
  @media (max-width: 600px) {
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 1rem 0.5rem;
  }
}

.intro-media {
  width: 320px;
  height: 180px;
  flex-shrink: 0;
  border-radius: 8px;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;

  // 调整内部 img
  :deep(img) {
    width: 100%;
    height: 100%;
    object-fit: cover;
    transition: transform 0.3s ease;

    &:hover {
      transform: scale(1.05);
    }
  }

  // 移动端适配
  @media (max-width: 600px) {
    width: 100%;
    max-width: 400px;
    height: auto;
    aspect-ratio: 16 / 9;
    margin-bottom: 0.5rem;
  }
}

.intro-content {
  flex: 1;
  min-width: 0;
}

.intro-title {
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  gap: 0.75rem;
  margin-bottom: 0.75rem;

  @media (max-width: 600px) {
    justify-content: center;
    margin-bottom: 1rem;
  }
}

.intro-name {
  font-weight: 700;
  color: #000000; // 浅色模式下改为纯黑
  font-size: 1.5rem;
  line-height: 1.1;

  .body--dark & {
    color: #ffffff;
  }

  @media (max-width: 600px) {
    font-size: 1.25rem;
  }
}

.intro-type {
  font-size: 0.85rem;
  color: #1976d2;
  background: rgba(25, 118, 210, 0.1);
  padding: 4px 8px;
  border-radius: 6px;
  font-weight: 500;
  white-space: nowrap;

  .body--dark & {
    color: #64b5f6;
    background: rgba(100, 181, 246, 0.15);
  }
}

.intro-description {
  :deep(.wiki-document) {
    font-size: 1rem;
    line-height: 1.2;
    color: #555;

    .body--dark & {
      color: #bbb;
    }
  }
}
</style>
