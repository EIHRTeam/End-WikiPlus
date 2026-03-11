<template>
  <router-view />
</template>

<script setup lang="ts">
import { onMounted } from 'vue';

onMounted(() => {
  // 仅在 Release 构建（生产环境）中禁用默认的右键菜单
  if (import.meta.env.PROD) {
    document.addEventListener('contextmenu', (e) => {
      e.preventDefault();
    });
  }
});
</script>

<style>
/* 全局滚动条样式 */
* {
  scrollbar-width: thin;
  scrollbar-color: rgba(128, 128, 128, 0.5) transparent;
}

/* Webkit 浏览器 (Chrome, Safari, Edge) */
*::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

*::-webkit-scrollbar-track {
  background: transparent;
}

*::-webkit-scrollbar-thumb {
  background-color: rgba(128, 128, 128, 0.5);
  border-radius: 4px;
  border: 2px solid transparent;
  background-clip: padding-box;
}

*::-webkit-scrollbar-thumb:hover {
  background-color: rgba(128, 128, 128, 0.7);
  border: 2px solid transparent;
  background-clip: padding-box;
}

*::-webkit-scrollbar-thumb:active {
  background-color: rgba(128, 128, 128, 0.9);
  border: 2px solid transparent;
  background-clip: padding-box;
}

/* 暗色模式下的滚动条 */
body.body--dark *::-webkit-scrollbar-thumb {
  background-color: rgba(255, 255, 255, 0.3);
}

body.body--dark *::-webkit-scrollbar-thumb:hover {
  background-color: rgba(255, 255, 255, 0.5);
}

body.body--dark *::-webkit-scrollbar-thumb:active {
  background-color: rgba(255, 255, 255, 0.7);
}

body.body--dark * {
  scrollbar-color: rgba(255, 255, 255, 0.3) transparent;
}
</style>
