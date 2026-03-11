<template>
  <span v-if="isInline" :class="['wiki-entry-inline', `show-type-${element.entry.showType}`]">
    <span class="wiki-entry-content">
      <img v-if="displayCover" :src="displayCover" class="wiki-entry-icon" />
      <span class="wiki-entry-name">{{ displayName }}</span>
    </span>
    <span v-if="showCount" class="entry-count">×{{ element.entry.count }}</span>
  </span>

  <div v-else class="wiki-entry-card">
    <span class="wiki-entry-content">
      <img v-if="displayCover" :src="displayCover" class="wiki-entry-icon" />
      <span class="wiki-entry-name">{{ displayName }}</span>
    </span>
    <div v-if="showCount" class="card-count">×{{ element.entry.count }}</div>
  </div>
</template>

<script setup lang="ts">
import { computed, inject, ref, watch, type Ref } from 'vue';
import type { EntryInline, CatalogItemMap } from '../../../types/wiki';
import { resolveAssetUrl } from '../../../utils/wiki-loader';

const props = defineProps<{
  element: EntryInline;
}>();

const catalogMap = inject<Ref<CatalogItemMap>>('wikiCatalogMap', ref({} as CatalogItemMap));

const entryId = computed(() => String(props.element.entry.id ?? '').trim());

const catalogEntry = computed(() => {
  const direct = catalogMap.value[entryId.value];
  if (direct) return direct;
  const numericKey = String(Number(entryId.value));
  return catalogMap.value[numericKey];
});

const displayName = computed(() => {
  return catalogEntry.value?.name || String(props.element.entry.id || '');
});

const resolvedCover = ref('');

watch(
  () => catalogEntry.value?.cover,
  async (newCover: string | undefined) => {
    if (newCover && typeof newCover === 'string') {
      resolvedCover.value = await resolveAssetUrl(newCover);
    } else {
      resolvedCover.value = '';
    }
  },
  { immediate: true },
);

const displayCover = computed(() => {
  return resolvedCover.value;
});

const showCount = computed(() => {
  return props.element.entry.count && props.element.entry.count !== '0';
});

const isInline = computed(() => props.element.entry.showType !== 'card-big');
</script>

<style scoped lang="scss">
.wiki-entry-inline {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 2px 4px;
  border-radius: 4px;
  vertical-align: middle;

  .wiki-entry-content {
    display: inline-flex;
    align-items: center;
    gap: 4px;
  }

  .wiki-entry-icon {
    width: 18px;
    height: 18px;
    border-radius: 3px;
    object-fit: cover;
  }

  .wiki-entry-name {
    max-width: 20em;
    font-size: inherit;
    line-height: 1.3;
  }

  .entry-count {
    font-size: 0.85em;
    opacity: 0.8;
  }

  &.show-type-card-big {
    display: inline-flex;
  }
}

.wiki-entry-card {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 6px 10px;
  background-color: #f5f5f5;
  border: 1px solid #e0e0e0;
  border-radius: 8px;

  .wiki-entry-content {
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }

  .wiki-entry-icon {
    width: 24px;
    height: 24px;
    border-radius: 4px;
    object-fit: cover;
  }

  .wiki-entry-name {
    color: #333;
    font-weight: 600;
  }

  .card-count {
    font-size: 0.85em;
    opacity: 0.8;
    color: #666 !important;
  }
}
</style>
