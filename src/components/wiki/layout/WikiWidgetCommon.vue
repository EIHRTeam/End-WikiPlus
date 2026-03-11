<template>
  <div v-if="widgetCommon" class="widget-common">
    <div v-if="tabs.length" class="tabbed-panels">
      <q-tabs
        v-model="activeTab"
        dense
        class="tab-header"
        active-color="primary"
        indicator-color="primary"
        inline-label
        :dark="$q.dark.isActive"
      >
        <q-tab v-for="tab in tabs" :key="tab.key" :name="tab.key" :label="tab.title">
          <ImageLoader
            v-if="tab.icon"
            :url="tab.icon"
            :alt="tab.title"
            :max-width="24"
            variant="inline"
            class="tab-icon"
          />
        </q-tab>
      </q-tabs>

      <q-tab-panels v-model="activeTab" animated class="tab-panels" :dark="$q.dark.isActive">
        <q-tab-panel v-for="tab in tabs" :key="tab.key" :name="tab.key" class="wiki-tab-panel">
          <WikiWidgetIntro v-if="tab.intro" :intro="tab.intro" :document-map="documentMap" />

          <WikiDocument v-if="tab.document" :document="tab.document" />
        </q-tab-panel>
      </q-tab-panels>
    </div>

    <div v-else class="stack-panels">
      <template v-for="panel in panels" :key="panel.key">
        <div class="stack-panel">
          <div v-if="panel.title || panel.icon" class="stack-title">
            <ImageLoader
              v-if="panel.icon"
              :url="panel.icon"
              :alt="panel.title"
              :max-width="24"
              variant="inline"
              class="tab-icon"
            />
            <span>{{ panel.title }}</span>
          </div>

          <WikiWidgetIntro v-if="panel.intro" :intro="panel.intro" :document-map="documentMap" />

          <WikiDocument v-if="panel.document" :document="panel.document" />
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useQuasar } from 'quasar';
import { useI18n } from 'vue-i18n';
import type { WidgetCommon, Document, WidgetTabData, WidgetIntro } from '../../../types/wiki';
import WikiDocument from '../WikiDocument.vue';
import WikiWidgetIntro from './WikiWidgetIntro.vue';
import ImageLoader from '../ImageLoader.vue';

const props = defineProps<{
  widgetCommon?: WidgetCommon;
  documentMap: Record<string, Document>;
}>();

const $q = useQuasar();
const { t } = useI18n();

type Panel = {
  key: string;
  title: string;
  icon: string;
  intro: WidgetIntro | null;
  document: Document | null;
};

function getTabData(tabId: string): WidgetTabData | undefined {
  return props.widgetCommon?.tabDataMap?.[tabId];
}

function getTabContentDocument(tabId: string): Document | null {
  const tabData = getTabData(tabId);
  if (!tabData?.content) return null;
  return props.documentMap[tabData.content] || null;
}

function buildTitle(tabId: string, title: string, index: number): string {
  if (title) return title;
  const tabData = getTabData(tabId);
  if (tabData?.intro?.name) return tabData.intro.name;
  if (tabData?.intro?.type) return tabData.intro.type;
  return t('wiki.tabFallbackTitle', { index: index + 1 });
}

const panels = computed<Panel[]>(() => {
  if (!props.widgetCommon) return [];
  const list = props.widgetCommon.tabList || [];

  if (list.length) {
    return list.map((tab, index) => {
      const data = getTabData(tab.tabId);
      return {
        key: tab.tabId,
        title: buildTitle(tab.tabId, tab.title, index),
        icon: tab.icon || '',
        intro: data?.intro ?? null,
        document: getTabContentDocument(tab.tabId),
      };
    });
  }

  const keys = Object.keys(props.widgetCommon.tabDataMap || {});
  return keys.map((key, index) => {
    const data = getTabData(key);
    return {
      key,
      title: buildTitle(key, '', index),
      icon: '',
      intro: data?.intro ?? null,
      document: getTabContentDocument(key),
    };
  });
});

const tabs = computed(() => {
  if (!props.widgetCommon) return [] as Panel[];
  const list = props.widgetCommon.tabList || [];
  if (!list.length) return [] as Panel[];
  return panels.value;
});

const activeTab = ref('');

watch(
  () => tabs.value,
  (nextTabs) => {
    if (!nextTabs.length) {
      activeTab.value = '';
      return;
    }
    const firstTab = nextTabs[0];
    if (!firstTab) {
      activeTab.value = '';
      return;
    }
    if (!activeTab.value || !nextTabs.find((tab) => tab.key === activeTab.value)) {
      activeTab.value = firstTab.key;
    }
  },
  { immediate: true },
);
</script>

<style scoped lang="scss">
.widget-common {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.tabbed-panels {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.tab-header {
  border-bottom: 1px solid var(--card-border);
}

.tab-panels {
  background: transparent;
}

.tab-panels :deep(.q-tab-panel) {
  background: transparent;
}

.wiki-tab-panel {
  background: transparent;
}

.tab-icon {
  width: 24px;
  height: 24px;
  object-fit: contain;
  margin-right: 0.5rem;
}

.stack-panels {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.stack-panel {
  padding-bottom: 0.5rem;
  border-bottom: 1px dashed var(--card-border);
}

.stack-panel:last-child {
  border-bottom: none;
}

.stack-title {
  font-weight: 600;
  // color: #333;
  margin-bottom: 0.75rem;
  line-height: 1.2;
  display: flex;
  align-items: center;
}
</style>
