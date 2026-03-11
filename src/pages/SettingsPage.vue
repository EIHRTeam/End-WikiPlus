<template>
  <q-page class="md2-settings-page">
    <div class="md2-settings-scroll">
      <div class="md2-subheader">{{ $t('settingsPage.appearanceTitle') }}</div>
      <q-list class="md2-settings-list">
        <q-item clickable v-ripple @click="showThemeDialog = true">
          <q-item-section avatar>
            <q-icon name="palette" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('settingsPage.themeLabel') }}</q-item-label>
            <q-item-label caption>{{ currentThemeLabel }}</q-item-label>
          </q-item-section>
        </q-item>

        <q-item clickable v-ripple @click="showLanguageDialog = true">
          <q-item-section avatar>
            <q-icon name="language" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('settingsPage.languageLabel') }}</q-item-label>
            <q-item-label caption>{{ currentLanguageLabel }}</q-item-label>
          </q-item-section>
        </q-item>
      </q-list>

      <q-separator class="md2-separator" />

      <div class="md2-subheader">{{ $t('settingsPage.behaviorTitle') }}</div>
      <q-list class="md2-settings-list">
        <q-item>
          <q-item-section avatar>
            <q-icon name="history" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('settingsPage.historyLimit') }}</q-item-label>
            <q-item-label caption>{{ $t('settingsPage.historyLimitHint') }}</q-item-label>
            <q-slider
              v-model="historyLimitModel"
              :min="3"
              :max="30"
              :step="1"
              label
              color="primary"
              class="q-mt-sm"
            />
          </q-item-section>
        </q-item>

        <q-item tag="label" v-ripple>
          <q-item-section avatar>
            <q-icon name="open_in_new" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('settingsPage.openFavoritesInNewStack') }}</q-item-label>
          </q-item-section>
          <q-item-section side>
            <q-toggle v-model="favoritesNewStackModel" color="primary" />
          </q-item-section>
        </q-item>
      </q-list>

      <q-separator class="md2-separator" />

      <q-list class="md2-settings-list">
        <q-item tag="label" v-ripple>
          <q-item-section avatar>
            <q-icon name="grid_view" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('settingsPage.showCollectionPieces') }}</q-item-label>
          </q-item-section>
          <q-item-section side>
            <q-toggle v-model="collectionPreviewPiecesModel" color="primary" />
          </q-item-section>
        </q-item>
      </q-list>

      <q-separator class="md2-separator" />

      <div class="md2-subheader">{{ $t('settingsPage.publicAccessTitle') }}</div>
      <div class="md2-public-banner">
        <q-icon name="public" size="20px" />
        <span>{{ $t('settingsPage.publicAccessDesc') }}</span>
      </div>

      <q-list class="md2-settings-list">
        <q-item>
          <q-item-section avatar>
            <q-icon name="fingerprint" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('settingsPage.didStatus') }}</q-item-label>
            <q-item-label caption class="ellipsis">{{ didLabel }}</q-item-label>
          </q-item-section>
          <q-item-section side>
            <q-chip
              dense
              size="sm"
              :color="sklandStore.hasDid ? 'positive' : 'warning'"
              text-color="white"
              :label="
                sklandStore.hasDid ? $t('settingsPage.didReady') : $t('settingsPage.didMissing')
              "
            />
          </q-item-section>
        </q-item>

        <q-item tag="label" v-ripple>
          <q-item-section avatar>
            <q-icon name="sync" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('settingsPage.autoFetchDid') }}</q-item-label>
            <q-item-label caption>{{ $t('settingsPage.autoFetchDidHint') }}</q-item-label>
          </q-item-section>
          <q-item-section side>
            <q-toggle
              :model-value="sklandStore.autoFetchDid"
              color="primary"
              @update:model-value="handleAutoFetchChange"
            />
          </q-item-section>
        </q-item>

        <q-item clickable v-ripple :disable="refreshingDid" @click="refreshDid">
          <q-item-section avatar>
            <q-icon name="refresh" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('settingsPage.refreshDid') }}</q-item-label>
            <q-item-label caption>{{ $t('settingsPage.refreshDidHint') }}</q-item-label>
          </q-item-section>
          <q-item-section side>
            <q-spinner v-if="refreshingDid" color="primary" size="20px" />
            <q-icon v-else name="chevron_right" color="grey" />
          </q-item-section>
        </q-item>
      </q-list>

      <q-separator class="md2-separator" />

      <template v-if="isMobilePlatform">
        <div class="md2-subheader">{{ $t('settingsPage.permissionsTitle') }}</div>
        <q-list class="md2-settings-list">
          <q-item clickable v-ripple @click="router.push({ name: 'permissions' })">
            <q-item-section avatar>
              <q-icon name="admin_panel_settings" color="primary" />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ $t('settingsPage.permissionsLabel') }}</q-item-label>
              <q-item-label caption>{{ $t('settingsPage.permissionsDesc') }}</q-item-label>
            </q-item-section>
            <q-item-section side>
              <q-icon name="chevron_right" color="grey" />
            </q-item-section>
          </q-item>
        </q-list>
        <q-separator class="md2-separator" />
      </template>

      <q-list class="md2-settings-list">
        <q-item clickable v-ripple @click="router.push({ name: 'about' })">
          <q-item-section avatar>
            <q-icon name="info" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('settingsPage.aboutLabel') }}</q-item-label>
            <q-item-label caption>{{ $t('settingsPage.aboutDesc') }}</q-item-label>
          </q-item-section>
          <q-item-section side>
            <q-icon name="chevron_right" color="grey" />
          </q-item-section>
        </q-item>
      </q-list>

      <div class="md2-bottom-spacer" />
    </div>

    <q-dialog v-model="showThemeDialog">
      <q-card class="md2-dialog-card">
        <q-card-section class="md2-dialog-title">
          {{ $t('settingsPage.themeLabel') }}
        </q-card-section>
        <q-list>
          <q-item
            v-for="opt in darkModeOptions"
            :key="opt.value"
            clickable
            v-ripple
            :active="darkModeModel === opt.value"
            active-class="text-primary"
            @click="
              darkModeModel = opt.value;
              showThemeDialog = false;
            "
          >
            <q-item-section avatar>
              <q-radio v-model="darkModeModel" :val="opt.value" color="primary" />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ opt.label }}</q-item-label>
            </q-item-section>
          </q-item>
        </q-list>
      </q-card>
    </q-dialog>

    <q-dialog v-model="showLanguageDialog">
      <q-card class="md2-dialog-card">
        <q-card-section class="md2-dialog-title">
          {{ $t('settingsPage.languageLabel') }}
        </q-card-section>
        <q-list>
          <q-item
            v-for="opt in languageOptions"
            :key="opt.value"
            clickable
            v-ripple
            :active="languageModel === opt.value"
            active-class="text-primary"
            @click="
              languageModel = opt.value;
              showLanguageDialog = false;
            "
          >
            <q-item-section avatar>
              <q-radio v-model="languageModel" :val="opt.value" color="primary" />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ opt.label }}</q-item-label>
            </q-item-section>
          </q-item>
        </q-list>
      </q-card>
    </q-dialog>
  </q-page>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useRouter } from 'vue-router';
import { useQuasar } from 'quasar';
import { useSettingsStore, type DarkMode, type Language } from 'src/stores/settings';
import { useSklandStore } from 'src/stores/skland';

type SelectOption<T> = {
  label: string;
  value: T;
};

const settingsStore = useSettingsStore();
const sklandStore = useSklandStore();
const router = useRouter();
const $q = useQuasar();
const { t, locale } = useI18n();

const isMobilePlatform = computed(() => $q.platform.is.mobile === true);

const showThemeDialog = ref(false);
const showLanguageDialog = ref(false);
const refreshingDid = ref(false);

const darkModeModel = computed<DarkMode>({
  get: () => settingsStore.darkMode,
  set: (value) => settingsStore.setDarkMode(value),
});

const darkModeOptions = computed<SelectOption<DarkMode>[]>(() => [
  { label: t('settingsPage.themeAuto'), value: 'auto' },
  { label: t('settingsPage.themeLight'), value: 'light' },
  { label: t('settingsPage.themeDark'), value: 'dark' },
]);

const currentThemeLabel = computed(() => {
  const opt = darkModeOptions.value.find((o) => o.value === darkModeModel.value);
  return opt?.label ?? '';
});

const languageOptions = computed<SelectOption<Language>[]>(() => [
  { label: t('settingsPage.languageZhCN'), value: 'zh-CN' },
  { label: t('settingsPage.languageZhTW'), value: 'zh-TW' },
  { label: t('settingsPage.languageEnUS'), value: 'en-US' },
  { label: t('settingsPage.languageJaJP'), value: 'ja-JP' },
]);

const languageModel = computed<Language>({
  get: () => settingsStore.language,
  set: (value) => {
    settingsStore.setLanguage(value);
    locale.value = value;
    document.documentElement.lang = value;
  },
});

const currentLanguageLabel = computed(() => {
  const opt = languageOptions.value.find((o) => o.value === languageModel.value);
  return opt?.label ?? '';
});

const historyLimitModel = computed<number>({
  get: () => settingsStore.historyLimit,
  set: (value) => settingsStore.setHistoryLimit(value),
});

const favoritesNewStackModel = computed<boolean>({
  get: () => settingsStore.favoritesOpensNewStack,
  set: (value) => settingsStore.setFavoritesOpensNewStack(value),
});

const collectionPreviewPiecesModel = computed<boolean>({
  get: () => settingsStore.circuitCollectionPreviewShowPieces,
  set: (value) => settingsStore.setCircuitCollectionPreviewShowPieces(value),
});

const didLabel = computed<string>(() => sklandStore.did.trim() || '-');

function handleAutoFetchChange(value: boolean) {
  sklandStore.setAutoFetchDid(value);
}

async function refreshDid() {
  if (refreshingDid.value) return;
  refreshingDid.value = true;
  try {
    await sklandStore.refreshDid();
    $q.notify({
      type: 'positive',
      message: t('settingsPage.refreshDidSuccess'),
    });
  } catch (error) {
    $q.notify({
      type: 'negative',
      message: t('settingsPage.refreshDidFailed', {
        error: error instanceof Error ? error.message : String(error),
      }),
    });
  } finally {
    refreshingDid.value = false;
  }
}

onMounted(() => {
  sklandStore.initialize();
});
</script>

<style scoped lang="scss">
.md2-settings-page {
  background: var(--q-page-background, #fafafa);
}

.body--dark .md2-settings-page {
  background: var(--q-page-background, #121212);
}

.md2-settings-scroll {
  max-width: 600px;
  margin: 0 auto;
  padding: 0 0 24px;
}

.md2-subheader {
  font-size: 14px;
  font-weight: 500;
  line-height: 48px;
  padding: 0 16px;
  color: $primary;
  letter-spacing: 0.01em;
}

.md2-settings-list {
  padding: 0;

  .q-item {
    min-height: 56px;
    padding: 8px 16px;
  }
}

.md2-separator {
  margin: 4px 0;
}

.md2-inline-input {
  max-width: 320px;
}

.md2-public-banner {
  display: flex;
  align-items: center;
  gap: 8px;
  margin: 0 16px 4px;
  padding: 12px 16px;
  border-radius: 4px;
  font-size: 14px;
  line-height: 20px;
  background: rgba(25, 118, 210, 0.08);
  color: #145ea8;
}

.body--dark .md2-public-banner {
  background: rgba(144, 202, 249, 0.12);
  color: #90caf9;
}

.md2-dialog-card {
  min-width: 280px;
  max-width: 360px;
  border-radius: 4px;
}

.md2-dialog-title {
  font-size: 20px;
  font-weight: 500;
  line-height: 28px;
  padding: 24px 24px 0;
  color: var(--text-primary, rgba(0, 0, 0, 0.87));
}

.body--dark .md2-dialog-title {
  color: rgba(255, 255, 255, 0.87);
}

.md2-bottom-spacer {
  height: calc(16px + env(safe-area-inset-bottom));
}
</style>
