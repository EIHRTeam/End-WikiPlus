<template>
  <div class="wiki-image-block">
    <ImageLoader v-bind="imageLoaderProps" />
    <div v-if="block.image.description" class="image-description">
      {{ block.image.description }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { ImageBlock } from '../../../types/wiki';
import ImageLoader from '../ImageLoader.vue';

const { block } = defineProps<{
  block: ImageBlock;
}>();

const imageLoaderProps = computed(() => {
  const base = {
    url: block.image.url,
    alt: block.image.description || '',
    variant: 'block' as const,
    enableViewer: true, // 启用图片查看器
  };

  if (block.image.clientWidth) {
    return {
      ...base,
      maxWidth: block.image.clientWidth,
    };
  }

  return base;
});
</script>

<style scoped lang="scss">
.wiki-image-block {
  margin: 1.5em 0;
  text-align: center;

  @media (max-width: 600px) {
    margin: 1em 0;
  }
}

.image-description {
  margin-top: 0.5em;
  font-size: 0.9em;
  color: #666;
  font-style: italic;
}
</style>
