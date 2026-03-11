<template>
  <TextInline v-if="textElement" :element="textElement" />
  <EntryInline v-else-if="entryElement" :element="entryElement" />
</template>

<script setup lang="ts">
import { computed } from 'vue';
import TextInline from './inline/TextInline.vue';
import EntryInline from './inline/EntryInline.vue';
import type {
  InlineElement,
  TextInline as TextInlineType,
  EntryInline as EntryInlineType,
} from '../../types/wiki';

const props = defineProps<{
  element: InlineElement;
}>();

const textElement = computed<TextInlineType | null>(() => {
  return props.element.kind === 'text' ? props.element : null;
});

const entryElement = computed<EntryInlineType | null>(() => {
  return props.element.kind === 'entry' ? props.element : null;
});
</script>
