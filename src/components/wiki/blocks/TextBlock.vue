<template>
  <component
    :is="tagName"
    :class="['wiki-text-block', `align-${block.align || 'left'}`, `text-${textKind}`]"
  >
    <WikiInlineElement
      v-for="(element, index) in block.text.inlineElements"
      :key="index"
      :element="element"
    />
  </component>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import WikiInlineElement from '../WikiInlineElement.vue';
import type { TextBlock } from '../../../types/wiki';

const props = defineProps<{
  block: TextBlock;
}>();

const textKind = computed(() => props.block.text.kind);

const tagName = computed(() => {
  const kind = textKind.value;

  // 标题类型
  if (kind.startsWith('heading')) {
    const level = kind.replace('heading', '');
    return `h${level}`;
  }

  // 其他特殊类型
  switch (kind) {
    case 'title':
      return 'h1';
    case 'subtitle':
      return 'h2';
    case 'caption':
      return 'div';
    default:
      return 'p';
  }
});
</script>

<style scoped lang="scss">
.wiki-text-block {
  margin-bottom: 0.8em;

  &:last-child {
    margin-bottom: 0;
  }

  &.align-left {
    text-align: left;
  }

  &.align-center {
    text-align: center;
  }

  &.align-right {
    text-align: right;
  }

  &.text-body {
    font-size: 15px;
    line-height: 1.2;
  }

  &.text-heading1 {
    font-size: 2em;
    font-weight: bold;
    margin: 1em 0 0.5em;
    line-height: 1.2;
  }

  &.text-heading2 {
    font-size: 1.75em;
    font-weight: bold;
    margin: 0.9em 0 0.5em;
    line-height: 1.2;
  }

  &.text-heading3 {
    font-size: 1.5em;
    font-weight: bold;
    margin: 0.8em 0 0.5em;
    line-height: 1.2;
  }

  &.text-heading4 {
    font-size: 1.25em;
    font-weight: bold;
    margin: 0.7em 0 0.5em;
    line-height: 1.2;
  }

  &.text-heading5 {
    font-size: 1.1em;
    font-weight: bold;
    margin: 0.6em 0 0.5em;
    line-height: 1.2;
  }

  &.text-heading6 {
    font-size: 1em;
    font-weight: bold;
    margin: 0.5em 0 0.5em;
    line-height: 1.2;
  }
}

h1,
h2,
h3,
h4,
h5,
h6 {
  &:first-child {
    margin-top: 0;
  }
}
</style>
