<template>
  <div v-if="widgetCommon" class="widget-table">
    <div v-for="(row, index) in widgetCommon.tableList" :key="index" class="table-row">
      <div class="table-label">{{ row.label }}</div>
      <div class="table-value">{{ row.value }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { WidgetCommon, Document } from '../../../types/wiki';

defineProps<{
  widgetCommon?: WidgetCommon;
  documentMap: Record<string, Document>;
}>();
</script>

<style scoped lang="scss">
.widget-table {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.table-row {
  display: grid;
  grid-template-columns: 120px 1fr;
  gap: 1rem;
  padding: 0.5rem 0;
  border-bottom: 1px solid var(--table-border-color);

  &:last-child {
    border-bottom: none;
  }
}

.table-label {
  // color: #666;
  font-weight: 500;
  line-height: 1.2;
}

.table-value {
  // color: #333;
  line-height: 1.2;
}
</style>
