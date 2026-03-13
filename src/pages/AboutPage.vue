<template>
  <q-page class="md2-about-page">
    <div class="md2-about-scroll">
      <!-- ═══ App Identity Header ═══ -->
      <div class="md2-about-header">
        <div class="md2-about-title">
          {{ $t('appName') }}<sup class="md2-about-sup">+</sup>
        </div>
        <div class="md2-about-subtitle">{{ $t('about.subtitle') }}</div>
        <div class="md2-about-edition">Public Version</div>
      </div>

      <!-- ═══ Section: Build Info ═══ -->
      <q-list class="md2-about-list">
        <q-item>
          <q-item-section avatar>
            <q-icon name="info_outline" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('about.version') }}</q-item-label>
            <q-item-label caption class="text-family-mono">{{ buildInfo.version || $t('about.defaultVersion') }}</q-item-label>
          </q-item-section>
        </q-item>

        <q-item v-if="buildInfo.buildTime">
          <q-item-section avatar>
            <q-icon name="schedule" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('about.buildTime') }}</q-item-label>
            <q-item-label caption class="text-family-mono">{{ buildInfo.buildTime }} (GMT+8)</q-item-label>
          </q-item-section>
        </q-item>
      </q-list>

      <q-separator class="md2-separator" />

      <!-- ═══ Section: Links ═══ -->
      <div class="md2-subheader">{{ $t('about.linksTitle') }}</div>
      <q-list class="md2-about-list">
        <q-item clickable v-ripple @click="openLink('https://github.com/EIHRTeam/End-WikiPlus')">
          <q-item-section avatar>
            <q-icon name="code" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('about.github') }}</q-item-label>
            <q-item-label caption>{{ $t('about.githubDesc') }}</q-item-label>
          </q-item-section>
          <q-item-section side>
            <q-icon name="open_in_new" color="grey" size="18px" />
          </q-item-section>
        </q-item>

        <q-item clickable v-ripple @click="openLink('https://github.com/EIHRTeam/End-WikiPlus/issues')">
          <q-item-section avatar>
            <q-icon name="feedback" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('about.feedback') }}</q-item-label>
            <q-item-label caption>{{ $t('about.feedbackDesc') }}</q-item-label>
          </q-item-section>
          <q-item-section side>
            <q-icon name="open_in_new" color="grey" size="18px" />
          </q-item-section>
        </q-item>
      </q-list>

      <q-separator class="md2-separator" />

      <!-- ═══ Section: Legal ═══ -->
      <div class="md2-subheader">{{ $t('about.legalTitle') }}</div>
      <q-list class="md2-about-list">
        <q-item clickable v-ripple @click="navigateTo('/user-agreement')">
          <q-item-section avatar>
            <q-icon name="gavel" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('about.userAgreement') }}</q-item-label>
            <q-item-label caption>{{ $t('about.userAgreementDesc') }}</q-item-label>
          </q-item-section>
          <q-item-section side>
            <q-icon name="chevron_right" color="grey" />
          </q-item-section>
        </q-item>

        <q-item clickable v-ripple @click="navigateTo('/privacy-policy')">
          <q-item-section avatar>
            <q-icon name="privacy_tip" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('about.privacyPolicy') }}</q-item-label>
            <q-item-label caption>{{ $t('about.privacyPolicyDesc') }}</q-item-label>
          </q-item-section>
          <q-item-section side>
            <q-icon name="chevron_right" color="grey" />
          </q-item-section>
        </q-item>

        <q-item clickable v-ripple @click="navigateTo('/license')">
          <q-item-section avatar>
            <q-icon name="description" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('about.license') }}</q-item-label>
            <q-item-label caption>{{ $t('about.licenseDesc') }}</q-item-label>
          </q-item-section>
          <q-item-section side>
            <q-icon name="chevron_right" color="grey" />
          </q-item-section>
        </q-item>

        <q-item clickable v-ripple @click="navigateTo('/third-party-licenses')">
          <q-item-section avatar>
            <q-icon name="policy" color="primary" />
          </q-item-section>
          <q-item-section>
            <q-item-label>{{ $t('about.thirdParty') }}</q-item-label>
            <q-item-label caption>{{ $t('about.thirdPartyDesc') }}</q-item-label>
          </q-item-section>
          <q-item-section side>
            <q-icon name="chevron_right" color="grey" />
          </q-item-section>
        </q-item>
      </q-list>

      <!-- ═══ Footer Disclaimer ═══ -->
      <div class="md2-about-footer">
        {{ $t('about.copyright') }}
        <br /><br />
        Neither this software nor the Endfield Industries Human Resource Team is affiliated with
        Shanghai Hypergryph Network Technology Co., Ltd. or its affiliated entities.
      </div>

      <div class="md2-bottom-spacer" />
    </div>
  </q-page>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';
import tauriConfig from '../../src-tauri/tauri.conf.json';

const router = useRouter();

const buildInfo: Record<string, string> = {
  version: tauriConfig.version,
  buildTime: __APP_BUILD_TIME__,
};

async function openLink(url: string) {
  try {
    await invoke('plugin:android-intent|openLink', { url });
  } catch {
    try {
      await open(url);
    } catch {
      window.open(url, '_blank');
    }
  }
}

function navigateTo(path: string) {
  router.push(path).catch(() => undefined);
}
</script>

<style scoped lang="scss">
/* ══════════════════════════════════════════════════════════════════════
   Material Design 2 — About Page
   Single-column list layout matching SettingsPage conventions.
   ══════════════════════════════════════════════════════════════════════ */

.md2-about-page {
  background: var(--q-page-background, #fafafa);
}

.body--dark .md2-about-page {
  background: var(--q-page-background, #121212);
}

.md2-about-scroll {
  max-width: 600px;
  margin: 0 auto;
  padding: 0 0 24px;
}

/* ── App Identity Header ─────────────────────────────────────────────── */
.md2-about-header {
  padding: 48px 16px 24px;
  text-align: center;
}

.md2-about-title {
  font-size: 32px;
  font-weight: 700;
  letter-spacing: -0.5px;
  line-height: 1.2;
  color: $primary;
}

.md2-about-sup {
  font-size: 0.6em;
  position: relative;
  top: -0.5em;
  vertical-align: baseline;
}

.md2-about-edition {
  margin-top: 6px;
  font-size: 12px;
  font-weight: 500;
  line-height: 18px;
  color: var(--text-tertiary, #888888);
}

.md2-about-subtitle {
  margin-top: 8px;
  font-size: 16px;
  line-height: 24px;
  color: var(--text-secondary, rgba(0, 0, 0, 0.54));
}

.body--dark .md2-about-subtitle {
  color: rgba(255, 255, 255, 0.54);
}

/* ── Subheader (reuse SettingsPage convention) ────────────────────────── */
.md2-subheader {
  font-size: 14px;
  font-weight: 500;
  line-height: 48px;
  padding: 0 16px;
  color: $primary;
  letter-spacing: 0.01em;
}

/* ── List ─────────────────────────────────────────────────────────────── */
.md2-about-list {
  padding: 0;

  .q-item {
    min-height: 56px;
    padding: 8px 16px;
  }
}

/* ── Separator ────────────────────────────────────────────────────────── */
.md2-separator {
  margin: 4px 0;
}

/* ── Mono font for version strings ────────────────────────────────────── */
.text-family-mono {
  font-family: 'Fira Code', 'Cascadia Code', 'Source Code Pro', monospace;
}

/* ── Footer ───────────────────────────────────────────────────────────── */
.md2-about-footer {
  margin-top: 32px;
  padding: 0 16px;
  text-align: center;
  font-size: 12px;
  line-height: 1.6;
  letter-spacing: 0.05em;
  color: var(--text-secondary, rgba(0, 0, 0, 0.38));
}

.body--dark .md2-about-footer {
  color: rgba(255, 255, 255, 0.38);
}

/* ── Bottom safe area spacer ──────────────────────────────────────────── */
.md2-bottom-spacer {
  height: calc(16px + env(safe-area-inset-bottom));
}
</style>
