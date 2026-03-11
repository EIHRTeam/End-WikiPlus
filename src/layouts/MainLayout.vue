<template>
  <q-layout view="hHh Lpr lFf">
    <q-header :class="$q.dark.isActive ? 'bg-dark text-white' : 'bg-white text-grey-9'" bordered>
      <q-toolbar class="q-py-xs relative-position">
        <q-btn
          class="gt-xs"
          flat
          dense
          round
          :icon="desktopDrawerMini ? 'menu' : 'menu_open'"
          :aria-label="$t('menu.toggleSidebar')"
          @click="toggleDesktopDrawerMini"
        />

        <div class="lt-sm absolute-center text-subtitle1 text-weight-bold">
          {{ $t('appName') }}<sup style="font-size: 0.75em; position: relative; top: -0.4em; vertical-align: baseline"
            >+</sup
          >
        </div>

        <q-toolbar-title class="gt-xs text-weight-bold row items-center q-pl-sm">
          <span class="text-h6 text-weight-bold"
            >{{ $t('appName') }}<sup
              style="font-size: 0.75em; position: relative; top: -0.4em; vertical-align: baseline"
              >+</sup
            ></span
          >
        </q-toolbar-title>

        <div class="gt-xs row q-gutter-sm items-center">
          <q-btn
            flat
            dense
            round
            :icon="isPageFullscreen ? 'fullscreen_exit' : 'fullscreen'"
            @click="togglePageFullscreen"
          >
            <q-tooltip>{{ $t('menu.fullscreen') }}</q-tooltip>
          </q-btn>
          <q-btn flat dense round :icon="$q.dark.isActive ? 'dark_mode' : 'light_mode'" @click="toggleTheme">
            <q-tooltip>{{ $t('menu.theme') }}</q-tooltip>
          </q-btn>
          <q-btn flat dense round icon="translate">
            <q-menu>
              <q-list style="min-width: 100px">
                <q-item v-for="lang in languageList" :key="lang.code" clickable @click="setLanguage(lang.code)">
                  <q-item-section>{{ lang.label }}</q-item-section>
                </q-item>
              </q-list>
            </q-menu>
          </q-btn>
        </div>
      </q-toolbar>
    </q-header>

    <q-footer v-if="$q.screen.lt.sm" bordered class="lt-sm bg-white text-grey-8">
      <q-tabs active-color="primary" indicator-color="transparent" align="justify" dense class="q-py-xs">
        <q-route-tab to="/" icon="list" :label="$t('menu.list')" size="sm" />
        <q-route-tab to="/wiki/render" icon="article" :label="$t('menu.render')" size="sm" />
        <q-route-tab to="/settings" icon="settings" :label="$t('menu.settings')" size="sm" />
      </q-tabs>
    </q-footer>

    <q-drawer
      v-if="$q.screen.gt.xs"
      :model-value="true"
      bordered
      :width="260"
      :mini-width="64"
      :mini="desktopDrawerMini"
      side="left"
      behavior="desktop"
    >
      <q-scroll-area class="fit">
        <q-list class="q-py-md desktop-nav-list">
          <q-item
            v-for="item in navItems"
            :key="item.key"
            v-bind="item.to ? { to: item.to } : {}"
            clickable
            :active="item.to ? isNavActive(item.to) : false"
            active-class="desktop-nav-active"
            class="desktop-nav-item"
          >
            <q-item-section avatar class="desktop-nav-icon">
              <q-icon :name="item.icon" size="24px" />
            </q-item-section>
            <q-item-section v-if="!desktopDrawerMini">
              <q-item-label>{{ item.label }}</q-item-label>
              <q-item-label v-if="item.desc" caption lines="1">{{ item.desc }}</q-item-label>
            </q-item-section>
          </q-item>
        </q-list>
      </q-scroll-area>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>

    <q-dialog v-model="showLegalConsentDialog" persistent no-esc-dismiss no-backdrop-dismiss>
      <q-card class="legal-consent-card">
        <q-card-section class="q-pb-none">
          <div class="text-h6">{{ $t('legalConsent.title') }}</div>
          <div class="text-body2 q-mt-sm">{{ $t('legalConsent.desc') }}</div>
        </q-card-section>

        <q-card-section class="q-pt-sm">
          <q-list bordered separator>
            <q-item clickable v-ripple @click="openLegalDocument('/user-agreement')">
              <q-item-section avatar>
                <q-icon name="gavel" color="primary" />
              </q-item-section>
              <q-item-section>{{ $t('legalConsent.userAgreement') }}</q-item-section>
              <q-item-section side>
                <q-btn flat dense color="primary" :label="$t('legalConsent.read')" />
              </q-item-section>
            </q-item>

            <q-item clickable v-ripple @click="openLegalDocument('/privacy-policy')">
              <q-item-section avatar>
                <q-icon name="privacy_tip" color="primary" />
              </q-item-section>
              <q-item-section>{{ $t('legalConsent.privacyPolicy') }}</q-item-section>
              <q-item-section side>
                <q-btn flat dense color="primary" :label="$t('legalConsent.read')" />
              </q-item-section>
            </q-item>

            <q-item clickable v-ripple @click="openLegalDocument('/license')">
              <q-item-section avatar>
                <q-icon name="description" color="primary" />
              </q-item-section>
              <q-item-section>{{ $t('legalConsent.softwareLicense') }}</q-item-section>
              <q-item-section side>
                <q-btn flat dense color="primary" :label="$t('legalConsent.read')" />
              </q-item-section>
            </q-item>
          </q-list>

          <q-checkbox
            v-model="legalConsentChecked"
            class="q-mt-md legal-consent-checkbox"
            :label="$t('legalConsent.checkbox')"
          />
        </q-card-section>

        <q-card-actions align="right">
          <q-btn color="primary" unelevated :label="$t('legalConsent.confirm')" @click="confirmLegalConsent" />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </q-layout>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { useQuasar } from 'quasar';
import { useI18n } from 'vue-i18n';
import { useSettingsStore, type Language } from 'src/stores/settings';
import { useDialogManager } from 'src/stores/dialogManager';
import { useSklandStore } from 'src/stores/skland';

const settingsStore = useSettingsStore();
const dialogManager = useDialogManager();
const sklandStore = useSklandStore();
const $q = useQuasar();
const route = useRoute();
const router = useRouter();
const { locale, t } = useI18n();

const showLegalConsentDialog = ref(false);
const legalConsentChecked = ref(false);
const resumeLegalConsentAfterReading = ref(false);

const LEGAL_CONSENT_DIALOG_ID = '__legal_consent_v1__';
const LEGAL_DOCUMENT_ROUTES = ['/user-agreement', '/privacy-policy', '/license'];

const languageList = computed(() => [
  { code: 'zh-CN' as Language, label: t('settingsPage.languageZhCN') },
  { code: 'zh-TW' as Language, label: t('settingsPage.languageZhTW') },
  { code: 'en-US' as Language, label: t('settingsPage.languageEnUS') },
  { code: 'ja-JP' as Language, label: t('settingsPage.languageJaJP') },
]);

function setLanguage(lang: Language) {
  settingsStore.setLanguage(lang);
  locale.value = lang;
  document.documentElement.lang = lang;
}

const isPageFullscreen = ref($q.fullscreen.isActive);

function handleFullscreenChange() {
  isPageFullscreen.value = $q.fullscreen.isActive;
}

function togglePageFullscreen() {
  if (!$q.fullscreen.isCapable) return;
  $q.fullscreen.toggle().catch(() => undefined);
}

function toggleTheme() {
  const newMode: 'light' | 'dark' = $q.dark.isActive ? 'light' : 'dark';
  $q.dark.set(newMode === 'dark');
  settingsStore.setDarkMode(newMode);
}

const desktopDrawerMini = ref(true);

function toggleDesktopDrawerMini() {
  desktopDrawerMini.value = !desktopDrawerMini.value;
}

interface NavItem {
  key: string;
  to?: string;
  icon: string;
  label: string;
  desc?: string;
}

const navItems = computed<NavItem[]>(() => [
  {
    key: 'list',
    to: '/',
    icon: 'list',
    label: t('menu.list'),
    desc: t('menu.listDesc'),
  },
  {
    key: 'render',
    to: '/wiki/render',
    icon: 'article',
    label: t('menu.render'),
    desc: t('menu.renderDesc'),
  },
  {
    key: 'settings',
    to: '/settings',
    icon: 'settings',
    label: t('menu.settings'),
    desc: t('menu.settingsDesc'),
  },
]);

function isNavActive(path: string): boolean {
  if (path === '/') return route.path === '/';
  return route.path.startsWith(path);
}

const hasAcceptedLegalConsent = computed(() =>
  settingsStore.acceptedStartupDialogs.includes(LEGAL_CONSENT_DIALOG_ID),
);

const isOnLegalDocumentRoute = computed(() =>
  LEGAL_DOCUMENT_ROUTES.some((path) => route.path.startsWith(path)),
);

function maybeShowLegalConsentDialog() {
  if (hasAcceptedLegalConsent.value) return;
  if (isOnLegalDocumentRoute.value) return;
  showLegalConsentDialog.value = true;
}

function openLegalDocument(path: string) {
  resumeLegalConsentAfterReading.value = true;
  showLegalConsentDialog.value = false;
  void router.push(path);
}

function confirmLegalConsent() {
  if (!legalConsentChecked.value) {
    $q.notify({
      type: 'warning',
      message: t('legalConsent.required'),
    });
    return;
  }

  settingsStore.addAcceptedStartupDialog(LEGAL_CONSENT_DIALOG_ID);
  showLegalConsentDialog.value = false;
  legalConsentChecked.value = false;
  resumeLegalConsentAfterReading.value = false;
}

onMounted(() => {
  document.documentElement.lang = locale.value;
  sklandStore.initialize();
  maybeShowLegalConsentDialog();
  (window as unknown as { jeiPackDialogLoaded?: () => void }).jeiPackDialogLoaded = () => {
    dialogManager.triggerProcess();
  };

  document.addEventListener('fullscreenchange', handleFullscreenChange);
  handleFullscreenChange();
});

onUnmounted(() => {
  delete (window as unknown as { jeiPackDialogLoaded?: () => void }).jeiPackDialogLoaded;
  document.removeEventListener('fullscreenchange', handleFullscreenChange);
});

watch(
  () => route.path,
  () => {
    if (hasAcceptedLegalConsent.value) return;
    if (isOnLegalDocumentRoute.value) return;
    if (resumeLegalConsentAfterReading.value || !showLegalConsentDialog.value) {
      showLegalConsentDialog.value = true;
      resumeLegalConsentAfterReading.value = false;
    }
  },
);
</script>

<style>
.q-header {
  border-bottom: 1px solid rgba(0, 0, 0, 0.05);
  padding-top: env(safe-area-inset-top);
}

body.body--dark .q-header {
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  padding-top: env(safe-area-inset-top);
}

.q-footer {
  padding-bottom: env(safe-area-inset-bottom);
}

.q-footer .q-tab__icon {
  font-size: 20px;
}

.q-footer .q-tab__label {
  font-size: 10px;
}

.desktop-nav-list .desktop-nav-item {
  border-radius: 8px;
  margin: 2px 8px;
  min-height: 48px;
  padding: 8px 12px;
  transition: background-color 0.2s ease;
}

.desktop-nav-list .desktop-nav-icon {
  min-width: 40px;
}

.q-drawer--mini .desktop-nav-list .desktop-nav-item {
  justify-content: center;
  margin: 2px 6px;
  padding: 8px;
}

.q-drawer--mini .desktop-nav-list .desktop-nav-icon {
  min-width: unset;
}

.desktop-nav-active {
  color: var(--q-primary);
}

body.body--dark .desktop-nav-active {
  background-color: rgba(255, 255, 255, 0.08);
}

body:not(.body--dark) .desktop-nav-active {
  background-color: rgba(0, 0, 0, 0.06);
}

.legal-consent-card {
  width: 92vw;
  max-width: 620px;
}

.legal-consent-checkbox {
  display: flex;
  align-items: flex-start;
}

.legal-consent-checkbox :deep(.q-checkbox__label) {
  line-height: 1.5;
}
</style>
