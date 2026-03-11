import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useWikiStore = defineStore('wiki', () => {
  //主要分类 Tab (Wiki, Guide, Archive)
  const selectedTopTab = ref<'wiki' | 'guide' | 'archive'>('wiki');

  // 搜索关键词
  const searchKeyword = ref('');

  return {
    selectedTopTab,
    searchKeyword,
  };
});
