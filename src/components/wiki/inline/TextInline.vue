<template>
  <span :class="textClasses" :style="textStyle">{{ element.text.text }}</span>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useQuasar } from 'quasar';
import type { TextInline } from '../../../types/wiki';
import { COLOR_MAP, DARK_COLOR_MAP } from '../../../types/wiki';

const props = defineProps<{
  element: TextInline;
}>();
const $q = useQuasar();

const textClasses = computed(() => ({
  'text-bold': props.element.bold,
  'text-italic': props.element.italic,
  'text-underline': props.element.underline,
  'text-strikethrough': props.element.strikethrough,
  'text-code': props.element.code,
}));

const textStyle = computed(() => {
  const style: Record<string, string> = {};

  if (props.element.color) {
    const isDark = $q.dark.isActive;
    const resolved = resolveRawColor(props.element.color, isDark);
    style.color = resolved;
  } else if (!$q.dark.isActive) {
    // 浅色模式下，如果没有指定颜色，默认为 inherit，允许父级样式穿透
    // style.color = 'var(--text-primary)';
  }

  return style;
});

function resolveRawColor(color: string, isDark: boolean): string {
  if (isDark && DARK_COLOR_MAP[color]) {
    return DARK_COLOR_MAP[color];
  }
  return COLOR_MAP[color] || color;
}
</script>

<style scoped lang="scss">
.text-bold {
  font-weight: bold;
  font-size: 1.05em;

  .body--dark & {
    color: #fff;
  }
}

.text-italic {
  font-style: italic;
}

.text-underline {
  text-decoration: underline;
}

.text-strikethrough {
  text-decoration: line-through;
}

.text-code {
  font-family: 'Courier New', monospace;
  background-color: #f5f5f5;
  padding: 2px 4px;
  border-radius: 3px;
  font-size: 0.9em;
}
</style>
