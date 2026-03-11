<template>
  <section class="wiki-chapter">
    <div class="chapter-title">
      <span class="title-text">{{ group.title }}</span>
      <div class="title-line"></div>
    </div>

    <div class="widgets-grid">
      <WikiWidget
        v-for="widget in group.widgets"
        :key="widget.id"
        :widget="widget"
        :widget-common-map="widgetCommonMap"
        :document-map="documentMap"
      />
    </div>
  </section>
</template>

<script setup lang="ts">
import type { ChapterGroup, WidgetCommon, Document } from '../../../types/wiki';
import WikiWidget from './WikiWidget.vue';

defineProps<{
  group: ChapterGroup;
  widgetCommonMap: Record<string, WidgetCommon>;
  documentMap: Record<string, Document>;
}>();
</script>

<style scoped lang="scss">
.wiki-chapter {
  margin-bottom: 2.5rem;
}

.chapter-title {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1.5rem;

  .title-text {
    font-size: 1.25rem;
    font-weight: 600;
    line-height: 1.2;
    // color: #333;
  }

  .title-line {
    flex: 1;
    height: 1px;
    // background: linear-gradient(to right, #d0d0d0, transparent);
  }
}

.widgets-grid {
  display: grid;
  grid-template-columns: repeat(12, 1fr);
  gap: 1rem;
}

@media (max-width: 1200px) {
  .widgets-grid {
    grid-template-columns: repeat(6, 1fr);
  }
}

@media (max-width: 768px) {
  .widgets-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}
</style>
