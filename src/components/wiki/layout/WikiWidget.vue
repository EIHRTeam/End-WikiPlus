<template>
  <q-card :class="['wiki-widget', sizeClass]" :dark="$q.dark.isActive">
    <q-card-section class="widget-header">
      <div class="widget-title">{{ widget.title }}</div>
    </q-card-section>

    <q-separator />

    <q-card-section class="widget-body">
      <template v-if="widgetCommon">
        <component
          :is="widgetComponent"
          :widget-common="widgetCommon"
          :document-map="documentMap"
        />
      </template>
      <div v-else class="widget-missing">{{ t('wiki.widgetMissingData') }}</div>
    </q-card-section>
  </q-card>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useQuasar } from 'quasar';
import { useI18n } from 'vue-i18n';
import type { WidgetRef, WidgetCommon, Document } from '../../../types/wiki';
import WikiWidgetTable from './WikiWidgetTable.vue';
import WikiWidgetCommon from './WikiWidgetCommon.vue';
import WikiWidgetAudio from './WikiWidgetAudio.vue';

const props = defineProps<{
  widget: WidgetRef;
  widgetCommonMap: Record<string, WidgetCommon>;
  documentMap: Record<string, Document>;
}>();

const $q = useQuasar();
const { t } = useI18n();

const widgetCommon = computed(() => props.widgetCommonMap[props.widget.id]);

const widgetComponent = computed(() => {
  if (!widgetCommon.value) {
    return 'div';
  }
  switch (widgetCommon.value.type) {
    case 'table':
      return WikiWidgetTable;
    case 'audio':
      return WikiWidgetAudio;
    case 'common':
    default:
      return WikiWidgetCommon;
  }
});

const sizeClass = computed(() => {
  switch (props.widget.size) {
    case 'small':
      return 'widget-size-small';
    case 'middle':
      return 'widget-size-middle';
    case 'large':
    default:
      return 'widget-size-large';
  }
});
</script>

<style scoped lang="scss">
.wiki-widget {
  border: 1px solid var(--card-border);
  border-radius: 8px;
  background: var(--card-bg);
  overflow: hidden;

  // 依赖全局定义的 CSS 变量自动切换，不再需要单独的 .body--dark 覆盖
  // .body--dark & {
  //   background: #1e1e1e;
  //   border-color: rgba(255, 255, 255, 0.12);
  // }
}

.widget-header {
  padding: 0.75rem 1rem;
  background: var(--card-header-bg);
}

.widget-title {
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
  line-height: 1.2;
}

.widget-body {
  padding: 1rem;
}

.widget-missing {
  color: #999;
  font-size: 0.9rem;
}

.widget-size-small {
  grid-column: span 4;
}

.widget-size-middle {
  grid-column: span 8;
}

.widget-size-large {
  grid-column: span 12;
}

@media (max-width: 1200px) {
  .widget-size-small {
    grid-column: span 3;
  }

  .widget-size-middle,
  .widget-size-large {
    grid-column: span 6;
  }
}

@media (max-width: 768px) {
  .widget-size-small,
  .widget-size-middle,
  .widget-size-large {
    grid-column: span 2;
  }
}
</style>
