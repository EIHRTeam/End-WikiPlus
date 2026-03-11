<template>
  <component :is="blockComponent" :block="block" :block-map="blockMap" />
</template>

<script setup lang="ts">
import { computed, defineAsyncComponent } from 'vue';
import type { Block } from '../../types/wiki';

const props = defineProps<{
  block: Block;
  blockMap: Record<string, Block>;
}>();

// 动态加载对应的组件
const blockComponent = computed(() => {
  const kind = props.block.kind;

  switch (kind) {
    case 'text':
      return defineAsyncComponent(() => import('./blocks/TextBlock.vue'));
    case 'table':
      return defineAsyncComponent(() => import('./blocks/TableBlock.vue'));
    case 'list':
      return defineAsyncComponent(() => import('./blocks/ListBlock.vue'));
    case 'image':
      return defineAsyncComponent(() => import('./blocks/ImageBlock.vue'));
    case 'quote':
      return defineAsyncComponent(() => import('./blocks/QuoteBlock.vue'));
    case 'horizontalLine':
      return defineAsyncComponent(() => import('./blocks/HorizontalLineBlock.vue'));
    default:
      return defineAsyncComponent(() => import('./blocks/UnknownBlock.vue'));
  }
});
</script>
