<template>
  <hr :class="['wiki-hr', `hr-kind-${block.horizontalLine.kind}`]" />
</template>

<script setup lang="ts">
import type { HorizontalLineBlock } from '../../../types/wiki';

defineProps<{
  block: HorizontalLineBlock;
}>();
</script>

<style scoped lang="scss">
.wiki-hr {
  margin: 1.5em 0;
  border: none;
  border-top: 1px solid #e0e0e0;

  &.hr-kind-1 {
    border-top-width: 1px;
    opacity: 0.3;
  }

  &.hr-kind-2 {
    border-top-width: 1px;
    opacity: 0.5;
  }

  &.hr-kind-3 {
    border-top-width: 2px;
    opacity: 0.7;
  }

  &.hr-kind-5 {
    border-top-width: 3px;
    opacity: 0.9;
  }
}
</style>
