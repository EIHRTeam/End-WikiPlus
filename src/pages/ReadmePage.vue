<template>
  <q-page padding class="doc-page">
    <q-card flat bordered :class="$q.dark.isActive ? 'bg-dark text-white' : ''">
      <q-card-section>
        <div class="doc-md" v-html="html"></div>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useQuasar } from 'quasar';
import MarkdownIt from 'markdown-it';
import readmeMd from 'src/assets/readme.generated.md?raw';

const $q = useQuasar();

const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true,
});

const html = computed(() => md.render(readmeMd));
</script>

<style scoped>
.doc-page {
  overflow: auto;
}

:deep(.body--dark) .doc-page {
  background: var(--q-dark-page);
}

.doc-md :deep(pre) {
  white-space: pre-wrap;
  word-break: break-word;
}

.doc-md :deep(h1) {
  font-size: 22px;
  line-height: 30px;
  margin: 0 0 12px;
}

.doc-md :deep(h2) {
  font-size: 18px;
  line-height: 26px;
  margin: 18px 0 10px;
}

.doc-md :deep(h3) {
  font-size: 16px;
  line-height: 24px;
  margin: 16px 0 8px;
}

.doc-md :deep(p),
.doc-md :deep(ul),
.doc-md :deep(ol) {
  margin: 8px 0;
}
</style>
