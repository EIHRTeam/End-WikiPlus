<template>
  <q-page class="wiki-list-page q-pa-sm q-pa-md-md">
    <!-- 二级分类 (仅当选中"百科"时显示自定义子分类) -->
    <!-- Mobile: Dense Tabs with outside arrows -->
    <div v-if="wikiStore.selectedTopTab === 'wiki'" class="q-mb-sm">
      <q-tabs
        v-model="selectedWikiSubId"
        dense
        class="text-grey"
        active-color="primary"
        indicator-color="primary"
        align="left"
        narrow-indicator
        :outside-arrows="$q.screen.lt.sm"
        :mobile-arrows="$q.screen.lt.sm"
      >
        <q-tab v-for="sub in wikiSubCategories" :key="sub.id" :name="sub.id" :label="sub.name" />
      </q-tabs>
    </div>

    <!-- 原有的动态子分类逻辑保留给其他 Top Tab (如果需要) -->
    <div v-else class="q-mb-sm">
      <q-tabs
        v-model="selectedMainTypeId"
        dense
        class="text-grey"
        active-color="primary"
        indicator-color="primary"
        align="left"
        narrow-indicator
        :outside-arrows="$q.screen.lt.sm"
        :mobile-arrows="$q.screen.lt.sm"
      >
        <q-tab name="all" :label="$t('list.all')" />
        <q-tab
          v-for="main in mainTypes"
          :key="main.id || 'unknown'"
          :name="main.id"
          :label="main.name"
        />
      </q-tabs>
    </div>

    <q-separator class="q-mb-md" />

    <!-- 三级分类/筛选 -->
    <div
      class="q-py-sm row q-gutter-sm items-center"
      v-if="wikiStore.selectedTopTab !== 'wiki' && subTypes.length > 0"
    >
      <div class="text-caption text-grey-7 q-mr-sm">
        {{ $t('list.subCategory') }}
      </div>
      <q-chip
        v-for="sub in subTypes"
        :key="sub.id || 'unknown'"
        :selected="selectedSubTypeId === sub.id"
        clickable
        @click="selectedSubTypeId = selectedSubTypeId === sub.id ? null : sub.id || null"
        color="primary"
        text-color="white"
        :outline="selectedSubTypeId !== sub.id"
      >
        {{ sub.name }}
      </q-chip>
    </div>

    <!-- 标签筛选区域 (优化为下拉菜单) -->
    <div v-if="filterSelectOptions.length > 0" class="row q-gutter-sm q-mb-md">
      <!-- 属性标签筛选 -->
      <q-select
        v-for="group in filterSelectOptions"
        :key="group.id"
        :options="group.options"
        :model-value="selectedTagMap[group.id] || []"
        @update:model-value="onGroupTagsChange(group.id, $event)"
        multiple
        use-chips
        outlined
        dense
        :label="group.name"
        :bg-color="$q.dark.isActive ? 'grey-9' : 'grey-1'"
        class="col-12 col-sm-auto"
        style="min-width: 200px"
        options-dense
      >
        <template v-slot:option="{ itemProps, opt, selected }">
          <q-item v-bind="itemProps" :class="{ 'text-primary': selected }">
            <q-item-section>
              <q-item-label>{{ opt.label }}</q-item-label>
            </q-item-section>
            <q-item-section side v-if="selected">
              <q-icon name="check" color="primary" />
            </q-item-section>
          </q-item>
        </template>
        <template v-slot:no-option>
          <q-item>
            <q-item-section class="text-grey">{{ $t('list.noOptions') }}</q-item-section>
          </q-item>
        </template>
      </q-select>
    </div>

    <!-- 搜索栏 -->
    <div class="q-mb-md">
      <q-input
        dense
        outlined
        v-model="wikiStore.searchKeyword"
        :placeholder="$t('list.searchPlaceholder')"
        :bg-color="$q.dark.isActive ? 'grey-9' : 'grey-1'"
      >
        <template v-slot:append>
          <q-icon name="search" />
        </template>
      </q-input>
    </div>

    <!-- 统计信息 (Desktop Only, or subtle text) -->
    <div class="text-right text-grey q-mb-sm text-caption">
      {{ $t('list.totalCount', { count: filteredItems.length }) }}
    </div>

    <!-- 加载状态 -->
    <div v-if="loading" class="row justify-center q-my-xl">
      <q-spinner-dots color="primary" size="40px" />
    </div>

    <!-- 列表展示 -->
    <div v-else-if="filteredItems.length > 0" class="row q-col-gutter-sm justify-center">
      <!-- Responsive Grid: col-6 (Mobile), col-sm-auto (Desktop fixed size via CSS) -->
      <div v-for="item in filteredItems" :key="item.itemId" class="col-6 col-sm-auto">
        <q-card
          class="cursor-pointer hover-effect column no-wrap full-height item-card"
          flat
          bordered
          @click="openItem(item)"
        >
          <!-- Card Image Wrapper -->
          <div class="relative-position col-grow overflow-hidden">
            <!-- Image with Aspect Ratio (Unified 3:4 portrait) -->
            <q-img
              :src="item.coverUrl"
              :ratio="3 / 4"
              class="bg-grey-2"
              loading="lazy"
              referrerpolicy="no-referrer"
            >
              <template v-slot:error>
                <div class="absolute-full flex flex-center bg-grey-3 text-grey-6">
                  <q-icon name="broken_image" size="md" />
                </div>
              </template>
              <template v-slot:loading>
                <q-spinner color="primary" />
              </template>

              <!-- Icons (Profession/Element) -->
              <div
                class="absolute-top-left column items-start"
                style="z-index: 10; padding: 4px; gap: 4px; background: none !important"
              >
                <q-img
                  v-if="item.profession"
                  :src="getAttributeIconUrl('profession', item.profession)"
                  class="attribute-icon"
                  :alt="item.profession"
                />
                <q-img
                  v-if="item.element"
                  :src="getAttributeIconUrl('element', item.element)"
                  class="attribute-icon"
                  :alt="item.element"
                />
              </div>

              <!-- Gradient Overlay for Text (Unified) -->
              <div
                class="absolute-bottom text-white q-pa-sm"
                style="
                  background: linear-gradient(
                    to top,
                    rgba(0, 0, 0, 0.9) 0%,
                    rgba(0, 0, 0, 0.4) 60%,
                    transparent 100%
                  );
                  pointer-events: none;
                "
              >
                <div class="text-subtitle2 text-weight-medium ellipsis">
                  {{ item.name }}
                </div>
              </div>
            </q-img>

            <!-- Offline Badge -->
            <q-badge
              v-if="item.status === 1"
              color="grey-9"
              text-color="white"
              class="absolute-top-right q-ma-xs"
              style="z-index: 10; opacity: 0.8; padding: 2px 6px"
            >
              {{ $t('list.offline') }}
            </q-badge>

            <!-- Rarity Bar -->
            <div
              v-if="item.rarity"
              class="absolute-bottom rarity-indicator-bar"
              :style="{ backgroundColor: getRarityColor(item.rarity) }"
            ></div>
          </div>
        </q-card>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-else class="row justify-center q-my-xl text-grey">
      <div v-if="errorState.hasError" class="text-center">
        <div class="text-negative q-mb-sm">{{ errorState.message }}</div>
        <q-btn
          flat
          color="primary"
          :label="$t('list.viewDetails')"
          @click="showErrorDetails = true"
        />
      </div>
      <div v-else>
        {{ $t('list.noResults') }}
      </div>
    </div>

    <!-- 错误详情对话框 -->
    <q-dialog v-model="showErrorDetails">
      <q-card style="min-width: 350px">
        <q-card-section>
          <div class="text-h6 text-negative">
            {{ $t('list.requestFailed') }}
          </div>
        </q-card-section>

        <q-card-section class="q-pt-none">
          <div class="text-body2 q-mb-sm">
            <strong>{{ $t('list.errorCode') }}</strong> {{ errorState.code }}
          </div>
          <div class="text-body2 q-mb-sm">
            <strong>{{ $t('list.statusCode') }}</strong> {{ errorState.status }}
          </div>
          <div
            class="text-caption bg-grey-2 q-pa-sm rounded-borders"
            style="white-space: pre-wrap; word-break: break-all"
          >
            {{ errorState.details }}
          </div>
        </q-card-section>

        <q-card-actions align="right">
          <q-btn flat :label="$t('close')" color="primary" v-close-popup />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </q-page>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useRoute, useRouter, type LocationQueryRaw } from 'vue-router';
import { useQuasar } from 'quasar';
import { useI18n } from 'vue-i18n';
import { loadWikiResource, resolveAssetUrl } from 'src/utils/wiki-loader';
import type { CatalogData, CatalogItem, TagNode } from 'src/types/wiki';
import { invoke } from '@tauri-apps/api/core';
import { useSklandStore } from 'src/stores/skland';
import { useWikiStore } from 'src/stores/wiki';

// 错误状态接口
interface ErrorState {
  hasError: boolean;
  message: string;
  code: string | number;
  status: string | number;
  details: string;
}

// 扩展 CatalogItem 以包含处理后的数据
interface DisplayItem extends CatalogItem {
  coverUrl: string;
  mainTypeId?: string | undefined;
  subTypeId?: string | undefined;
  mainTypeName?: string | undefined;
  subTypeName?: string | undefined;
  // 解析后的属性（用于 UI 显示）
  rarity?: number | undefined;
  profession?: string | undefined;
  element?: string | undefined;
}

// q-select option format
interface SelectOption {
  label: string;
  value: string;
}

const router = useRouter();
const route = useRoute();
const $q = useQuasar();
const { t } = useI18n();
const sklandStore = useSklandStore();
const wikiStore = useWikiStore();
const loading = ref(true);
const allItems = ref<DisplayItem[]>([]);
const catalogData = ref<CatalogData | null>(null);

// 错误状态管理
const errorState = ref<ErrorState>({
  hasError: false,
  message: '',
  code: '-',
  status: '-',
  details: '',
});
const showErrorDetails = ref(false);

const selectedWikiSubId = ref<string>('char'); // Default to 'char' (干员)

// 映射表：百科下的子分类
interface WikiSubCategory {
  id: string;
  name: string;
  mainId: string;
  subId: string;
}
const wikiSubCategories = computed<WikiSubCategory[]>(() => [
  { id: 'char', name: t('list.categories.char'), mainId: '1', subId: '1' },
  { id: 'weapon', name: t('list.categories.weapon'), mainId: '1', subId: '2' },
  { id: 'equip', name: t('list.categories.equip'), mainId: '1', subId: '4' },
  {
    id: 'weapon_matrix',
    name: t('list.categories.weapon_matrix'),
    mainId: '1',
    subId: '7',
  },
  { id: 'threat', name: t('list.categories.threat'), mainId: '1', subId: '3' },
  { id: 'device', name: t('list.categories.device'), mainId: '1', subId: '5' },
  { id: 'item', name: t('list.categories.item'), mainId: '1', subId: '6' },
  {
    id: 'valuable',
    name: t('list.categories.valuable'),
    mainId: '1',
    subId: '16',
  },
  {
    id: 'blueprint',
    name: t('list.categories.blueprint'),
    mainId: '1',
    subId: '18',
  },
  { id: 'quest', name: t('list.categories.quest'), mainId: '1', subId: '8' },
  {
    id: 'activity',
    name: t('list.categories.activity'),
    mainId: '1',
    subId: '9',
  },
]);

const selectedMainTypeId = ref<string>('all');
const selectedSubTypeId = ref<string | null>(null);
const selectedTagMap = ref<Record<string, SelectOption[]>>({});
const isHydratingFromRoute = ref(false);

// 创建中文到英文文件名的映射
const attributeNameMap: Record<string, string> = {
  // 职业 (Professions)
  近卫: 'guard',
  术师: 'caster',
  突击: 'striker',
  先锋: 'vangard',
  重装: 'defender',
  辅助: 'supporter',
  // 属性 (Elements)
  灼热: 'heat',
  电磁: 'electric',
  寒冷: 'cryo',
  自然: 'nature',
  物理: 'physical',
};

// 使用 import.meta.glob 批量导入图标资源
// eager: true 确保直接返回模块内容（包含默认导出的 URL），而不是动态导入函数
const iconModules = import.meta.glob('../assets/icon/**/*.png', {
  eager: true,
  import: 'default',
});

function resetError() {
  errorState.value = {
    hasError: false,
    message: '',
    code: '-',
    status: '-',
    details: '',
  };
}

function getAttributeIconUrl(type: 'profession' | 'element', value: string): string {
  if (!value) return '';
  // Map Chinese name to English filename
  const englishName = attributeNameMap[value];
  if (!englishName) {
    console.warn(`No English mapping found for attribute ${type}: ${value}`);
    return '';
  }

  const subDir = type === 'profession' ? 'profession' : 'element';
  // 构建相对于当前文件的路径，与 import.meta.glob 的键匹配
  const path = `../assets/icon/${subDir}/prop_${type}_${englishName}.png`;

  const iconUrl = iconModules[path];

  if (typeof iconUrl === 'string') {
    return iconUrl;
  } else {
    console.warn(
      `Could not find icon for ${type}: ${value} (mapped to ${englishName}) at path: ${path}`,
    );
    return '';
  }
}

function handleError(e: unknown) {
  const details = String(e);
  let status: string | number = '-';

  if (typeof e === 'string') {
    // 尝试解析后端返回的错误字符串 (通常格式为 "Request failed with status: 403" 或其他)
    if (e.includes('status:')) {
      const parts = e.split('status:');
      const statusPart = parts[1];
      if (statusPart) {
        status = statusPart.trim();
      }
    }
  }

  errorState.value = {
    hasError: true,
    message: t('list.requestFailed'),
    code: '-', // 后端目前没返回具体业务 code，暂时留空
    status,
    details,
  };
}

// 监听百科子分类变化，更新 selectedMainTypeId 和 selectedSubTypeId 以触发加载
// Now watching wikiStore.selectedTopTab instead of local selectedTopTab

watch(
  () => wikiStore.selectedTopTab,
  (topTab) => {
    if (topTab === 'wiki') {
      const category = wikiSubCategories.value.find(
        (c: WikiSubCategory) => c.id === selectedWikiSubId.value,
      );
      if (category) {
        selectedMainTypeId.value = category.mainId;
        selectedSubTypeId.value = category.subId;
      }
    }
  },
  { immediate: true },
);

// Also watch selectedWikiSubId to update when sub-tab changes

watch(selectedWikiSubId, (wikiSub) => {
  if (wikiStore.selectedTopTab === 'wiki') {
    const category = wikiSubCategories.value.find((c: WikiSubCategory) => c.id === wikiSub);
    if (category) {
      selectedMainTypeId.value = category.mainId;
      selectedSubTypeId.value = category.subId;
    }
  }
});

// 主分类列表
const mainTypes = computed(() => {
  if (!catalogData.value || !catalogData.value.data || !catalogData.value.data.catalog) {
    return [];
  }
  return catalogData.value.data.catalog;
});

// 当前选中的主分类对象
const currentMainType = computed(() => {
  if (selectedMainTypeId.value === 'all') return null;
  return mainTypes.value.find((m) => m.id === selectedMainTypeId.value);
});

// 子分类列表 (基于选中的主分类)
const subTypes = computed(() => {
  if (!currentMainType.value) {
    // 如果选了全部，或者没选主分类，暂时不显示子分类，或者显示所有子分类？
    // 简单起见，只有选中主分类才显示子分类
    return [];
  }
  return currentMainType.value.typeSub || [];
});

// 当前选中的子分类对象
const currentSubType = computed(() => {
  if (!selectedSubTypeId.value) return null;
  // 在所有可能的子分类中查找
  for (const main of mainTypes.value) {
    if (!main.id) continue;
    const sub = main.typeSub?.find((s) => s.id === selectedSubTypeId.value);
    if (sub) return sub;
  }
  return null;
});

// 标签筛选树 (基于选中的子分类)
const currentFilterTree = computed(() => {
  return currentSubType.value?.filterTagTree || [];
});

// Computed property to transform filter tree into q-select options
const filterSelectOptions = computed(() => {
  if (!currentFilterTree.value) return [];
  return currentFilterTree.value.map((group) => ({
    id: group.id,
    name: group.name,
    options:
      group.children?.map((child) => ({
        label: child.name,
        value: child.id,
      })) || [],
  }));
});

function hasQueryKey(key: string): boolean {
  return Object.prototype.hasOwnProperty.call(route.query, key);
}

function getSingleQueryValue(value: unknown): string {
  if (typeof value === 'string') return value;
  if (Array.isArray(value)) {
    const first = value.find((item): item is string => typeof item === 'string');
    return first ?? '';
  }
  return '';
}

function parseCsvQueryValue(value: unknown): string[] {
  const raw = getSingleQueryValue(value);
  if (!raw) return [];
  return raw
    .split(',')
    .map((part) => part.trim())
    .filter((part) => part.length > 0);
}

function normalizeQueryForCompare(query: Record<string, unknown>): Record<string, string> {
  const normalized: Record<string, string> = {};
  for (const [key, value] of Object.entries(query)) {
    if (typeof value === 'string') {
      if (value.length > 0) normalized[key] = value;
      continue;
    }
    if (Array.isArray(value)) {
      const merged = value.filter((item): item is string => typeof item === 'string').join(',');
      if (merged.length > 0) normalized[key] = merged;
    }
  }
  return normalized;
}

function isSameQuery(a: Record<string, unknown>, b: Record<string, unknown>): boolean {
  const normalizedA = normalizeQueryForCompare(a);
  const normalizedB = normalizeQueryForCompare(b);
  const keysA = Object.keys(normalizedA).sort();
  const keysB = Object.keys(normalizedB).sort();

  if (keysA.length !== keysB.length) return false;
  return keysA.every((key, index) => key === keysB[index] && normalizedA[key] === normalizedB[key]);
}

function onGroupTagsChange(groupId: string, values: unknown) {
  const next = { ...selectedTagMap.value };
  const normalized = Array.isArray(values)
    ? values.filter((value): value is SelectOption => {
        if (typeof value !== 'object' || value === null) return false;
        const maybeOption = value as Partial<SelectOption>;
        return typeof maybeOption.label === 'string' && typeof maybeOption.value === 'string';
      })
    : [];

  if (normalized.length === 0) {
    delete next[groupId];
  } else {
    next[groupId] = normalized;
  }

  selectedTagMap.value = next;
}

function buildListQueryFromState(): LocationQueryRaw {
  const nextQuery: LocationQueryRaw = {};
  if (selectedWikiSubId.value) {
    nextQuery.sub = selectedWikiSubId.value;
  }
  if (selectedMainTypeId.value) {
    nextQuery.main = selectedMainTypeId.value;
  }
  if (selectedSubTypeId.value) {
    nextQuery.typeSub = selectedSubTypeId.value;
  }

  const keyword = wikiStore.searchKeyword.trim();
  if (keyword) {
    nextQuery.kw = keyword;
  }

  for (const group of filterSelectOptions.value) {
    const selected = selectedTagMap.value[group.id] || [];
    if (selected.length > 0) {
      nextQuery[`tag_${group.id}`] = selected.map((opt) => opt.value).join(',');
    }
  }

  return nextQuery;
}

async function syncRouteQueryFromState() {
  if (isHydratingFromRoute.value) return;

  const nextQuery = buildListQueryFromState();
  if (isSameQuery(route.query as Record<string, unknown>, nextQuery)) {
    return;
  }

  isHydratingFromRoute.value = true;
  try {
    await router.replace({
      name: 'wiki-list',
      query: nextQuery,
    });
  } finally {
    isHydratingFromRoute.value = false;
  }
}

function hydrateStateFromRouteQuery() {
  if (isHydratingFromRoute.value) return;

  isHydratingFromRoute.value = true;
  try {
    const sub = getSingleQueryValue(route.query.sub).trim();
    if (sub && wikiSubCategories.value.some((category) => category.id === sub)) {
      selectedWikiSubId.value = sub;
    }

    if (hasQueryKey('main')) {
      const main = getSingleQueryValue(route.query.main).trim();
      if (main) {
        selectedMainTypeId.value = main;
      }
    }

    if (hasQueryKey('typeSub')) {
      const typeSub = getSingleQueryValue(route.query.typeSub).trim();
      selectedSubTypeId.value = typeSub || null;
    }

    if (hasQueryKey('kw')) {
      wikiStore.searchKeyword = getSingleQueryValue(route.query.kw);
    } else {
      wikiStore.searchKeyword = '';
    }

    const nextTagMap: Record<string, SelectOption[]> = {};
    for (const group of filterSelectOptions.value) {
      const values = parseCsvQueryValue(route.query[`tag_${group.id}`]);
      if (values.length === 0) continue;

      nextTagMap[group.id] = values.map((value) => {
        return (
          group.options.find((option) => option.value === value) ?? {
            label: value,
            value,
          }
        );
      });
    }
    selectedTagMap.value = nextTagMap;
  } finally {
    isHydratingFromRoute.value = false;
  }
}

const listStateSignature = computed(() => {
  const tagSignature = Object.entries(selectedTagMap.value)
    .sort(([a], [b]) => a.localeCompare(b))
    .map(
      ([groupId, options]) =>
        `${groupId}:${options
          .map((opt) => opt.value)
          .sort()
          .join(',')}`,
    )
    .join('|');

  return [
    selectedWikiSubId.value,
    selectedMainTypeId.value,
    selectedSubTypeId.value ?? '',
    wikiStore.searchKeyword.trim(),
    tagSignature,
  ].join('::');
});

watch(
  () => route.fullPath,
  () => {
    hydrateStateFromRouteQuery();
  },
  { immediate: true },
);

watch(
  () => filterSelectOptions.value.map((group) => group.id).join(','),
  () => {
    hydrateStateFromRouteQuery();
  },
);

watch(listStateSignature, () => {
  void syncRouteQueryFromState();
});

// 监听主分类变化，重置子分类和标签
watch(selectedMainTypeId, () => {
  if (!isHydratingFromRoute.value) {
    selectedTagMap.value = {};
  }

  // 只有在非 Wiki 模式下才执行自动选中第一个子分类的逻辑
  // 因为 Wiki 模式下 selectedSubTypeId 由 selectedWikiSubId 驱动
  if (wikiStore.selectedTopTab !== 'wiki' && !isHydratingFromRoute.value) {
    // 尝试自动选中第一个子分类
    const main = currentMainType.value;
    const firstSub = main?.typeSub?.[0];
    if (firstSub) {
      selectedSubTypeId.value = firstSub.id || null;
    } else {
      selectedSubTypeId.value = null;
    }
  }
});

function getRarityColor(rarity: number | undefined): string {
  if (!rarity) return 'transparent';
  switch (rarity) {
    case 6:
      return '#FF7100';
    case 5:
      return '#FFCC00';
    case 4:
      return '#B380FF';
    case 3:
      return '#33C2FF';
    case 2:
      return '#B4D945';
    case 1:
      return '#B2B2B2';
    default:
      return 'transparent';
  }
}

// 监听子分类变化，重置标签
watch(selectedSubTypeId, () => {
  if (!isHydratingFromRoute.value) {
    selectedTagMap.value = {};
  }
});

// 过滤列表
const filteredItems = computed(() => {
  let items = allItems.value;

  // 1. 主分类筛选
  if (selectedMainTypeId.value !== 'all') {
    items = items.filter((item) => item && item.mainTypeId === selectedMainTypeId.value);
  }

  // 2. 子分类筛选
  if (selectedSubTypeId.value) {
    items = items.filter((item) => item && item.subTypeId === selectedSubTypeId.value);
  }

  // 3. 标签筛选 (组内 OR, 组间 AND)
  const tagSelections = Object.values(selectedTagMap.value).filter(
    (group) => group && group.length > 0,
  );
  if (tagSelections.length > 0) {
    items = items.filter((item) => {
      if (!item || !item.tagIds) return false;

      // 检查每一个有选中项的筛选组
      for (const groupSelections of tagSelections) {
        // 组内逻辑：并集 (OR)
        // 只要条目包含当前组中任意一个选中的 tagId，就通过该组的筛选
        const matchesInGroup = groupSelections.some(
          (option) => item.tagIds && item.tagIds.includes(option.value),
        );

        // 如果条目不满足当前这个组的任何一个选项，则直接过滤掉（组间 AND）
        if (!matchesInGroup) return false;
      }
      return true;
    });
  }

  // 4. 关键词搜索
  if (wikiStore.searchKeyword) {
    const keyword = wikiStore.searchKeyword.toLowerCase().trim();
    items = items.filter(
      (item) =>
        item &&
        ((item.name && item.name.toLowerCase().includes(keyword)) ||
          (item.itemId && item.itemId.toLowerCase().includes(keyword))),
    );
  }

  return items;
});

// 工具函数：为 Promise 添加超时机制
function withTimeout<T>(
  promise: Promise<T>,
  ms: number,
  errorMessage = t('list.requestTimeout'),
): Promise<T> {
  return new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error(errorMessage)), ms);
    promise.then(
      (res) => {
        clearTimeout(timer);
        resolve(res);
      },
      (err) => {
        clearTimeout(timer);
        reject(err instanceof Error ? err : new Error(String(err)));
      },
    );
  });
}

// 加载数据
onMounted(async () => {
  sklandStore.initialize();
  loading.value = true;
  resetError();
  try {
    let data: CatalogData | null = null;

    // 尝试调用 Tauri 命令获取最新目录
    if (window.__TAURI_INTERNALS__) {
      try {
        // 确保有有效的设备 ID（缺失时自动走公共 DID 注册流程）
        let dId = '';
        try {
          dId = await sklandStore.ensureDid();
        } catch (didError) {
          console.warn('Failed to ensure device ID for catalog:', didError);
        }

        const initialCategory = wikiSubCategories.value.find(
          (c: WikiSubCategory) => c.id === selectedWikiSubId.value,
        );

        const requestCatalog = async (deviceId: string) =>
          await invoke<string>('fetch_wiki_catalog', {
            mainId: initialCategory?.mainId ?? null,
            subId: initialCategory?.subId ?? null,
            dId: deviceId,
            userAgent: sklandStore.userAgent,
          });

        let jsonStr = '';
        try {
          jsonStr = await withTimeout(
            requestCatalog(dId),
            15000,
            t('list.fetchCatalogTimeout', { seconds: 15 }),
          );
        } catch (error) {
          // 设备 ID 无效时，刷新 DID 后重试一次
          const message = error instanceof Error ? error.message : String(error);
          if (
            message.includes('设备信息无效') ||
            message.toLowerCase().includes('device id') ||
            message.toLowerCase().includes('device info')
          ) {
            dId = await sklandStore.refreshDid();
            jsonStr = await withTimeout(
              requestCatalog(dId),
              15000,
              t('list.fetchCatalogTimeout', { seconds: 15 }),
            );
          } else {
            throw error;
          }
        }

        data = JSON.parse(jsonStr);

        // 检查业务错误码
        if (data && data.code !== 0 && data.code !== undefined) {
          throw new Error(data.message || t('list.apiError', { code: data.code }));
        }
      } catch (e) {
        console.warn('Failed to fetch wiki catalog via Tauri:', e);
        throw e; // 直接抛出，防止执行后续可能导致 TypeError 的代码
      }
    } else {
      // 非 Tauri 环境，回退到本地资源
      data = await loadWikiResource<CatalogData>('temp/catalog/full.json');
    }

    if (!data) {
      throw new Error(t('list.noData'));
    }

    catalogData.value = data;

    if (data.data?.catalog) {
      for (const category of data.data.catalog) {
        if (category.typeSub) {
          for (const subType of category.typeSub) {
            // 构建标签映射表 (id -> name)
            const tagMap = new Map<string, string>();
            if (subType.filterTagTree) {
              const walk = (nodes: TagNode[]) => {
                for (const node of nodes) {
                  if (node.id && node.name) tagMap.set(node.id, node.name);
                  if (node.children) walk(node.children);
                }
              };
              walk(subType.filterTagTree);
            }

            if (subType.items) {
              for (const item of subType.items) {
                let coverUrl = '';
                if (item.brief?.cover) {
                  coverUrl = await resolveAssetUrl(item.brief.cover);
                }

                // 解析属性
                let rarity = 0;
                let profession = '';
                let element = '';

                if (item.tagIds) {
                  for (const tagId of item.tagIds) {
                    const tagName = tagMap.get(tagId);
                    if (!tagName) continue;

                    // 简单启发式规则：
                    // 星级：通常包含“星”字
                    if (tagName.includes('星')) {
                      const match = tagName.match(/(\d+)星/);
                      if (match && match[1]) rarity = parseInt(match[1]);
                    }
                    // 职业：近卫, 术师, 突击, 先锋, 重装, 辅助
                    else if (['近卫', '术师', '突击', '先锋', '重装', '辅助'].includes(tagName)) {
                      profession = tagName;
                    }
                    // 属性：灼热, 电磁, 寒冷, 自然, 物理
                    else if (['灼热', '电磁', '寒冷', '自然', '物理'].includes(tagName)) {
                      element = tagName;
                    }
                  }
                }

                allItems.value.push({
                  ...item,
                  coverUrl,
                  mainTypeId: category.id || undefined,
                  subTypeId: subType.id || undefined,
                  mainTypeName: category.name || undefined,
                  subTypeName: subType.name || undefined,
                  rarity: rarity || undefined,
                  profession: profession || undefined,
                  element: element || undefined,
                });
              }
            }
          }
        }
      }
    }

    // 数据加载完成后再次基于 URL 回填筛选项，确保筛选器可用时正确恢复。
    hydrateStateFromRouteQuery();
    if (hasQueryKey('status')) {
      void syncRouteQueryFromState();
    }

    // 默认选中第一个主分类 (如果有)
    // 只有在非 Wiki 模式下且 URL 未指定 main 时自动选中第一个
    if (wikiStore.selectedTopTab !== 'wiki' && !hasQueryKey('main')) {
      const firstMain = mainTypes.value[0];
      if (firstMain && firstMain.id) {
        selectedMainTypeId.value = firstMain.id;
      }
    }
  } catch (error) {
    console.error('Failed to load wiki catalog:', error);
    // Mobile debugging: show alert on error
    $q.notify({
      type: 'negative',
      message: `${t('error')}: ${error instanceof Error ? error.message : String(error)}`,
      timeout: 10000,
    });
    handleError(error);
  } finally {
    loading.value = false;
  }
});

// 监听子分类变化，加载具体数据
watch(selectedSubTypeId, async (newSubId) => {
  if (!newSubId || !selectedMainTypeId.value) return;

  loading.value = true;
  resetError();

  try {
    let data: CatalogData | null = null;

    if (window.__TAURI_INTERNALS__) {
      try {
        // 确保有有效的设备 ID
        let dId = '';
        try {
          dId = await sklandStore.ensureDid();
        } catch (didError) {
          console.warn('Failed to ensure device ID for sub-catalog:', didError);
        }

        const requestSubCatalog = async (deviceId: string) =>
          await invoke<string>('fetch_wiki_catalog', {
            mainId: selectedMainTypeId.value,
            subId: newSubId,
            dId: deviceId,
            userAgent: sklandStore.userAgent,
          });

        let jsonStr = '';
        try {
          jsonStr = await withTimeout(
            requestSubCatalog(dId),
            15000,
            t('list.fetchSubCatalogTimeout', { seconds: 15 }),
          );
        } catch (error) {
          // 设备 ID 无效时，刷新 DID 后重试一次
          const message = error instanceof Error ? error.message : String(error);
          if (
            message.includes('设备信息无效') ||
            message.toLowerCase().includes('device id') ||
            message.toLowerCase().includes('device info')
          ) {
            dId = await sklandStore.refreshDid();
            jsonStr = await withTimeout(
              requestSubCatalog(dId),
              15000,
              t('list.fetchSubCatalogTimeout', { seconds: 15 }),
            );
          } else {
            throw error;
          }
        }

        try {
          data = JSON.parse(jsonStr);
        } catch (e) {
          void e;
          const snippet = jsonStr.substring(0, 500);
          throw new Error(t('list.invalidJsonResponse', { snippet: `${snippet}...` }));
        }

        // 检查业务错误码
        if (data && data.code !== 0 && data.code !== undefined) {
          throw new Error(data.message || t('list.apiError', { code: data.code }));
        }
      } catch (e) {
        console.warn('Failed to fetch sub-catalog via Tauri:', e);
        throw e; // 直接抛出，阻止回退
      }
    }

    if (!data) return; // 如果数据为空，提前退出，loading会正确关闭

    if (data.data?.catalog) {
      const newItems: DisplayItem[] = [];

      for (const category of data.data.catalog) {
        if (category.typeSub) {
          for (const subType of category.typeSub) {
            // 构建标签映射表 (id -> name)
            const tagMap = new Map<string, string>();
            if (subType.filterTagTree) {
              const walk = (nodes: TagNode[]) => {
                for (const node of nodes) {
                  if (node.id && node.name) tagMap.set(node.id, node.name);
                  if (node.children) walk(node.children);
                }
              };
              walk(subType.filterTagTree);
            }

            if (subType.items) {
              for (const item of subType.items) {
                let coverUrl = '';
                if (item.brief?.cover) {
                  coverUrl = await resolveAssetUrl(item.brief.cover);
                }

                // 解析属性
                let rarity = 0;
                let profession = '';
                let element = '';

                if (item.tagIds) {
                  for (const tagId of item.tagIds) {
                    const tagName = tagMap.get(tagId);
                    if (!tagName) continue;

                    // 简单启发式规则：
                    // 星级：通常包含“星”字
                    if (tagName.includes('星')) {
                      const match = tagName.match(/(\d+)星/);
                      if (match && match[1]) rarity = parseInt(match[1]);
                    }
                    // 职业：近卫, 术师, 突击, 先锋, 重装, 辅助
                    else if (['近卫', '术师', '突击', '先锋', '重装', '辅助'].includes(tagName)) {
                      profession = tagName;
                    }
                    // 属性：灼热, 电磁, 寒冷, 自然, 物理
                    else if (['灼热', '电磁', '寒冷', '自然', '物理'].includes(tagName)) {
                      element = tagName;
                    }
                  }
                }

                newItems.push({
                  ...item,
                  coverUrl,
                  mainTypeId: category.id || undefined,
                  subTypeId: subType.id || undefined,
                  mainTypeName: category.name || undefined,
                  subTypeName: subType.name || undefined,
                  rarity: rarity || undefined,
                  profession: profession || undefined,
                  element: element || undefined,
                });
              }
            }
          }
        }
      }
      allItems.value = newItems;
    }
  } catch (e) {
    console.error('Failed to load sub-catalog:', e);
    // Mobile debugging
    $q.notify({
      type: 'negative',
      message: t('list.subCatalogError', { message: e instanceof Error ? e.message : String(e) }),
      timeout: 10000,
    });
  } finally {
    loading.value = false;
  }
});

function openItem(item: DisplayItem) {
  const itemId = item.itemId?.trim();
  if (!itemId) return;

  void router.push({
    name: 'wiki-item',
    params: { itemId },
    query: route.query,
  });
}
</script>

<style scoped lang="scss">
.wiki-list-page {
  background-color: var(--q-page-background);
}

.hover-effect {
  transition:
    transform 0.2s ease,
    box-shadow 0.2s ease;

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }
}

.bg-transparent-black {
  background: rgba(0, 0, 0, 0.6);
}
</style>
