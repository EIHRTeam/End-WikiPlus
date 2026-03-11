<template>
  <component :is="listTag" class="wiki-list">
    <li v-for="itemId in block.list.itemIds" :key="itemId" class="wiki-list-item">
      <template v-for="entry in getItemBlockEntries(itemId)" :key="entry.id">
        <WikiBlock :block="entry.block" :block-map="blockMap" />
      </template>
    </li>
  </component>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import WikiBlock from '../WikiBlock.vue';
import type { ListBlock, Block } from '../../../types/wiki';

const props = defineProps<{
  block: ListBlock;
  blockMap: Record<string, Block>;
}>();

const listTag = computed(() => {
  return props.block.list.kind === 'ordered' ? 'ol' : 'ul';
});

function getItemChildren(itemId: string): string[] {
  const item = props.block.list.itemMap[itemId];
  return item?.childIds || [];
}

function getItemBlockEntries(itemId: string): Array<{ id: string; block: Block }> {
  return getItemChildren(itemId)
    .map((id) => ({ id, block: props.blockMap[id] }))
    .filter((entry): entry is { id: string; block: Block } => Boolean(entry.block));
}
</script>

<style scoped lang="scss">
.wiki-list {
  margin: 1em 0;
  padding-left: 2em;

  &:first-child {
    margin-top: 0;
  }

  &:last-child {
    margin-bottom: 0;
  }
}

.wiki-list-item {
  margin-bottom: 0.5em;

  &:last-child {
    margin-bottom: 0;
  }
}
</style>
