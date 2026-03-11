<template>
  <div class="wiki-document">
    <WikiBlock
      v-for="entry in topBlocks"
      :key="entry.id"
      :block="entry.block"
      :block-map="document.blockMap"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import WikiBlock from './WikiBlock.vue';
import type { Document, Block } from '../../types/wiki';

const props = defineProps<{
  document: Document;
}>();

const topBlocks = computed(() => {
  return props.document.blockIds
    .map((id) => ({ id, block: props.document.blockMap[id] }))
    .filter((entry): entry is { id: string; block: Block } => Boolean(entry.block));
});
</script>

<style scoped lang="scss">
.wiki-document {
  line-height: 1.2;
  font-size: 15px;
  // color: #1f1f1f;
}
</style>
