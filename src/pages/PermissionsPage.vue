<template>
  <q-page class="md2-settings-page">
    <div class="md2-settings-scroll">
      <!-- ═══ Header ═══ -->
      <div class="md2-perm-header">
        <q-icon name="admin_panel_settings" size="40px" color="primary" />
        <div class="md2-perm-header-text">
          <div class="md2-perm-title">{{ t('permissionsPage.title') }}</div>
          <div class="md2-perm-subtitle">{{ t('permissionsPage.subtitle') }}</div>
        </div>
      </div>

      <!-- ═══ Platform-specific permission list ═══ -->

      <!-- Android permissions -->
      <template v-if="platformInfo === 'android'">
        <div class="md2-subheader">{{ t('permissionsPage.mediaAccessTitle') }}</div>
        <q-list class="md2-settings-list">
          <!-- Read Media Images -->
          <q-item tag="label" v-ripple @click="handleToggle('readMediaImages')">
            <q-item-section avatar>
              <q-icon name="photo_library" color="primary" />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ t('permissionsPage.readMediaImages') }}</q-item-label>
              <q-item-label caption>{{ t('permissionsPage.readMediaImagesDesc') }}</q-item-label>
            </q-item-section>
            <q-item-section side>
              <q-toggle :model-value="permissions.readMediaImages" color="primary" @click.prevent />
            </q-item-section>
          </q-item>

          <!-- Read Media Audio -->
          <q-item tag="label" v-ripple @click="handleToggle('readMediaAudio')">
            <q-item-section avatar>
              <q-icon name="audiotrack" color="primary" />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ t('permissionsPage.readMediaAudio') }}</q-item-label>
              <q-item-label caption>{{ t('permissionsPage.readMediaAudioDesc') }}</q-item-label>
            </q-item-section>
            <q-item-section side>
              <q-toggle :model-value="permissions.readMediaAudio" color="primary" @click.prevent />
            </q-item-section>
          </q-item>

          <!-- Read Media Video -->
          <q-item tag="label" v-ripple @click="handleToggle('readMediaVideo')">
            <q-item-section avatar>
              <q-icon name="videocam" color="primary" />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ t('permissionsPage.readMediaVideo') }}</q-item-label>
              <q-item-label caption>{{ t('permissionsPage.readMediaVideoDesc') }}</q-item-label>
            </q-item-section>
            <q-item-section side>
              <q-toggle :model-value="permissions.readMediaVideo" color="primary" @click.prevent />
            </q-item-section>
          </q-item>

          <!-- Write External Storage (only shown on older Android) -->
          <q-item v-if="!permissions.writeExternalStorage" tag="label" v-ripple
            @click="handleToggle('writeExternalStorage')">
            <q-item-section avatar>
              <q-icon name="save" color="primary" />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ t('permissionsPage.writeStorage') }}</q-item-label>
              <q-item-label caption>{{ t('permissionsPage.writeStorageDesc') }}</q-item-label>
            </q-item-section>
            <q-item-section side>
              <q-toggle :model-value="permissions.writeExternalStorage" color="primary" @click.prevent />
            </q-item-section>
          </q-item>
        </q-list>
      </template>

      <!-- iOS permissions -->
      <template v-else-if="platformInfo === 'ios'">
        <div class="md2-subheader">{{ t('permissionsPage.mediaAccessTitle') }}</div>
        <q-list class="md2-settings-list">
          <!-- Photo Library (Add Only) -->
          <q-item tag="label" v-ripple @click="handleToggle('photoLibraryAddOnly')">
            <q-item-section avatar>
              <q-icon name="photo_library" color="primary" />
            </q-item-section>
            <q-item-section>
              <q-item-label>{{ t('permissionsPage.photoLibrary') }}</q-item-label>
              <q-item-label caption>{{ t('permissionsPage.photoLibraryDesc') }}</q-item-label>
            </q-item-section>
            <q-item-section side>
              <q-toggle :model-value="permissions.photoLibraryAddOnly" color="primary" @click.prevent />
            </q-item-section>
          </q-item>
        </q-list>
      </template>

      <!-- Not supported (desktop / web) -->
      <template v-else>
        <div class="md2-perm-unsupported">
          <q-icon name="info" color="grey-6" size="24px" />
          <span>{{ t('permissionsPage.notSupported') }}</span>
        </div>
      </template>

      <q-separator class="md2-separator" v-if="platformInfo === 'android' || platformInfo === 'ios'" />

      <!-- ═══ Open System Settings ═══ -->
      <q-list class="md2-settings-list" v-if="platformInfo === 'android' || platformInfo === 'ios'">
        <q-item clickable v-ripple @click="openSystemSettings">
          <q-item-section avatar>
            <q-icon name="settings" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ t('permissionsPage.openSystemSettings') }}</q-item-label>
            <q-item-label caption>{{ t('permissionsPage.openSystemSettingsDesc') }}</q-item-label>
          </q-item-section>
          <q-item-section side>
            <q-icon name="open_in_new" color="grey" />
          </q-item-section>
        </q-item>
      </q-list>

      <!-- Status info -->
      <div class="md2-perm-info" v-if="platformInfo === 'android'">
        <q-icon name="info_outline" size="16px" />
        <span>{{ t('permissionsPage.androidNote') }}</span>
      </div>
      <div class="md2-perm-info" v-if="platformInfo === 'ios'">
        <q-icon name="info_outline" size="16px" />
        <span>{{ t('permissionsPage.iosNote') }}</span>
      </div>

      <div class="md2-bottom-spacer" />
    </div>
  </q-page>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, onActivated } from 'vue';
import { useQuasar } from 'quasar';
import { useI18n } from 'vue-i18n';
import { invoke } from '@tauri-apps/api/core';
import {
  isAndroidTauriRuntime,
  isIOSTauriRuntime,
} from 'src/utils/android-media';

const $q = useQuasar();
const { t } = useI18n();

// Platform detection
const platformInfo = ref<'android' | 'ios' | 'other'>('other');

// Permission states
const permissions = reactive<Record<string, boolean>>({
  readMediaImages: false,
  readMediaAudio: false,
  readMediaVideo: false,
  writeExternalStorage: true,
  photoLibraryAddOnly: false,
  photoLibraryReadWrite: false,
  fileAccess: true,
});

async function refreshPermissions() {
  try {
    const result = await invoke<Record<string, unknown>>('plugin:android-intent|checkPermissions');
    const platform = result.platform as string;

    if (platform === 'android') {
      platformInfo.value = 'android';
      permissions.readMediaImages = result.readMediaImages === true;
      permissions.readMediaAudio = result.readMediaAudio === true;
      permissions.readMediaVideo = result.readMediaVideo === true;
      permissions.writeExternalStorage = result.writeExternalStorage === true;
    } else if (platform === 'ios') {
      platformInfo.value = 'ios';
      permissions.photoLibraryAddOnly = result.photoLibraryAddOnly === true;
      permissions.photoLibraryReadWrite = result.photoLibraryReadWrite === true;
      permissions.fileAccess = result.fileAccess === true;
    }
  } catch {
    // Plugin not available (desktop/web)
    platformInfo.value = 'other';
  }
}

async function handleToggle(permissionKey: string) {
  const isGranted = permissions[permissionKey];

  if (isGranted) {
    // Already granted → open system settings
    await openSystemSettings();
  } else {
    // Not granted → request permissions
    try {
      const result = await invoke<Record<string, unknown>>('plugin:android-intent|requestPermissions');
      if (result.allGranted === true && result.requested === false) {
        $q.notify({
          type: 'positive',
          message: t('permissionsPage.alreadyGranted'),
          timeout: 1500,
        });
      } else if (result.requested === true) {
        $q.notify({
          type: 'info',
          message: t('permissionsPage.permissionRequested'),
          timeout: 2000,
        });
      }
    } catch (error: unknown) {
      const msg = error instanceof Error ? error.message : String(error);
      $q.notify({
        type: 'negative',
        message: t('permissionsPage.requestFailed'),
        caption: msg,
        timeout: 2600,
      });
    }

    // Refresh state after a short delay to catch permission dialog result
    setTimeout(() => void refreshPermissions(), 1000);
  }
}

async function openSystemSettings() {
  try {
    await invoke('plugin:android-intent|openAppSettings');
  } catch (error: unknown) {
    const msg = error instanceof Error ? error.message : String(error);
    $q.notify({
      type: 'negative',
      message: t('permissionsPage.openSettingsFailed'),
      caption: msg,
      timeout: 2600,
    });
  }
}

onMounted(() => {
  if (isAndroidTauriRuntime()) {
    platformInfo.value = 'android';
  } else if (isIOSTauriRuntime()) {
    platformInfo.value = 'ios';
  }
  void refreshPermissions();
});

// Refresh when navigating back from system settings
onActivated(() => {
  void refreshPermissions();
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

/* ── Permission Header ──────────────────────────────────────────────── */
.md2-perm-header {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 24px 16px 8px;
}

.md2-perm-header-text {
  flex: 1;
  min-width: 0;
}

.md2-perm-title {
  font-size: 18px;
  font-weight: 500;
  line-height: 28px;
  color: var(--text-primary, rgba(0, 0, 0, 0.87));
}

.body--dark .md2-perm-title {
  color: rgba(255, 255, 255, 0.87);
}

.md2-perm-subtitle {
  font-size: 14px;
  line-height: 20px;
  color: var(--text-secondary, rgba(0, 0, 0, 0.54));
}

.body--dark .md2-perm-subtitle {
  color: rgba(255, 255, 255, 0.54);
}

/* ── Subheader ──────────────────────────────────────────────────────── */
.md2-subheader {
  font-size: 14px;
  font-weight: 500;
  line-height: 48px;
  padding: 0 16px;
  color: $primary;
  letter-spacing: 0.01em;
}

/* ── Settings list ──────────────────────────────────────────────────── */
.md2-settings-list {
  padding: 0;

  .q-item {
    min-height: 56px;
    padding: 8px 16px;
  }
}

/* ── Separator ──────────────────────────────────────────────────────── */
.md2-separator {
  margin: 4px 0;
}

/* ── Info text ──────────────────────────────────────────────────────── */
.md2-perm-info {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  margin: 12px 16px;
  padding: 12px 16px;
  border-radius: 4px;
  font-size: 13px;
  line-height: 18px;
  background: rgba(0, 0, 0, 0.04);
  color: rgba(0, 0, 0, 0.54);
}

.body--dark .md2-perm-info {
  background: rgba(255, 255, 255, 0.06);
  color: rgba(255, 255, 255, 0.54);
}

/* ── Unsupported banner ─────────────────────────────────────────────── */
.md2-perm-unsupported {
  display: flex;
  align-items: center;
  gap: 12px;
  margin: 24px 16px;
  padding: 16px;
  border-radius: 4px;
  font-size: 14px;
  background: rgba(0, 0, 0, 0.04);
  color: rgba(0, 0, 0, 0.54);
}

.body--dark .md2-perm-unsupported {
  background: rgba(255, 255, 255, 0.06);
  color: rgba(255, 255, 255, 0.54);
}

/* ── Bottom safe area ───────────────────────────────────────────────── */
.md2-bottom-spacer {
  height: calc(16px + env(safe-area-inset-bottom));
}
</style>
