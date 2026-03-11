import { defineStore } from 'pinia';
import { Dark } from 'quasar';

export type DarkMode = 'dark' | 'light' | 'auto';
export type Language = 'zh-CN' | 'zh-TW' | 'en-US' | 'ja-JP';
type CircuitEditorPiecePanelState = {
  x: number;
  y: number;
  width: number;
  height: number;
  minimized: boolean;
  docked: boolean;
};

// 探测浏览器语言
function detectBrowserLanguage(): Language {
  const browserLang = navigator.language;
  if (browserLang.startsWith('zh')) return 'zh-CN';
  if (browserLang.startsWith('ja')) return 'ja-JP';
  return 'en-US';
}

export const useSettingsStore = defineStore('settings', {
  state: () => {
    const defaults = {
      historyLimit: 6,
      debugLayout: false,
      debugNavPanel: false,
      recipeViewMode: 'panel' as 'dialog' | 'panel',
      recipeSlotShowName: true,
      selectedPack: 'aef',
      favoritesCollapsed: false,
      panelCollapsed: false,
      darkMode: 'light' as DarkMode,
      language: detectBrowserLanguage(),
      debugPanelPos: { x: 10, y: 10 },
      acceptedStartupDialogs: [] as string[],
      completedTutorial: false,
      favoritesOpensNewStack: false,
      circuitCollectionPreviewShowPieces: false,
      circuitEditorPiecePanel: {
        x: 16,
        y: 120,
        width: 420,
        height: 620,
        minimized: false,
        docked: false,
      } as CircuitEditorPiecePanelState,
    };
    try {
      const raw = localStorage.getItem('jei.settings');
      if (!raw) {
        Dark.set('auto');
        return defaults;
      }
      const parsed = JSON.parse(raw) as Partial<typeof defaults>;

      // 应用深色模式设置
      if (parsed.darkMode) {
        Dark.set(parsed.darkMode === 'auto' ? 'auto' : parsed.darkMode === 'dark');
      } else {
        // 默认行为
        Dark.set(true);
      }

      const language: Language =
        parsed.language === 'zh-CN' ||
        parsed.language === 'zh-TW' ||
        parsed.language === 'en-US' ||
        parsed.language === 'ja-JP'
          ? parsed.language
          : defaults.language;
      const recipeViewMode: 'dialog' | 'panel' =
        parsed.recipeViewMode === 'panel' ? 'panel' : 'dialog';
      const panelParsed = parsed.circuitEditorPiecePanel;
      const circuitEditorPiecePanel =
        panelParsed &&
        typeof panelParsed.x === 'number' &&
        Number.isFinite(panelParsed.x) &&
        typeof panelParsed.y === 'number' &&
        Number.isFinite(panelParsed.y) &&
        typeof panelParsed.width === 'number' &&
        Number.isFinite(panelParsed.width) &&
        typeof panelParsed.height === 'number' &&
        Number.isFinite(panelParsed.height) &&
        typeof panelParsed.minimized === 'boolean'
          ? {
              x: panelParsed.x,
              y: panelParsed.y,
              width: panelParsed.width,
              height: panelParsed.height,
              minimized: panelParsed.minimized,
              docked:
                typeof panelParsed.docked === 'boolean'
                  ? panelParsed.docked
                  : defaults.circuitEditorPiecePanel.docked,
            }
          : defaults.circuitEditorPiecePanel;
      return {
        historyLimit:
          typeof parsed.historyLimit === 'number' ? parsed.historyLimit : defaults.historyLimit,
        debugLayout:
          typeof parsed.debugLayout === 'boolean' ? parsed.debugLayout : defaults.debugLayout,
        debugNavPanel:
          typeof parsed.debugNavPanel === 'boolean' ? parsed.debugNavPanel : defaults.debugNavPanel,
        recipeViewMode,
        recipeSlotShowName:
          typeof parsed.recipeSlotShowName === 'boolean'
            ? parsed.recipeSlotShowName
            : defaults.recipeSlotShowName,
        selectedPack:
          typeof parsed.selectedPack === 'string' ? parsed.selectedPack : defaults.selectedPack,
        favoritesCollapsed:
          typeof parsed.favoritesCollapsed === 'boolean'
            ? parsed.favoritesCollapsed
            : defaults.favoritesCollapsed,
        panelCollapsed:
          typeof parsed.panelCollapsed === 'boolean'
            ? parsed.panelCollapsed
            : defaults.panelCollapsed,
        darkMode:
          parsed.darkMode === 'dark' || parsed.darkMode === 'light' || parsed.darkMode === 'auto'
            ? parsed.darkMode
            : defaults.darkMode,
        language,
        debugPanelPos:
          parsed.debugPanelPos &&
          typeof parsed.debugPanelPos.x === 'number' &&
          typeof parsed.debugPanelPos.y === 'number'
            ? parsed.debugPanelPos
            : defaults.debugPanelPos,
        acceptedStartupDialogs: Array.isArray(parsed.acceptedStartupDialogs)
          ? parsed.acceptedStartupDialogs.filter((x): x is string => typeof x === 'string')
          : defaults.acceptedStartupDialogs,
        completedTutorial:
          typeof parsed.completedTutorial === 'boolean'
            ? parsed.completedTutorial
            : defaults.completedTutorial,
        favoritesOpensNewStack:
          typeof parsed.favoritesOpensNewStack === 'boolean'
            ? parsed.favoritesOpensNewStack
            : defaults.favoritesOpensNewStack,
        circuitCollectionPreviewShowPieces:
          typeof parsed.circuitCollectionPreviewShowPieces === 'boolean'
            ? parsed.circuitCollectionPreviewShowPieces
            : defaults.circuitCollectionPreviewShowPieces,
        circuitEditorPiecePanel,
      };
    } catch {
      Dark.set(true);
      return defaults;
    }
  },
  actions: {
    setHistoryLimit(limit: number) {
      this.historyLimit = limit;
      this.save();
    },
    setDebugLayout(enabled: boolean) {
      this.debugLayout = enabled;
      this.save();
    },
    setDebugNavPanel(enabled: boolean) {
      this.debugNavPanel = enabled;
      this.save();
    },
    setRecipeViewMode(mode: 'dialog' | 'panel') {
      this.recipeViewMode = mode;
      this.save();
    },
    setRecipeSlotShowName(enabled: boolean) {
      this.recipeSlotShowName = enabled;
      this.save();
    },
    setSelectedPack(packId: string) {
      this.selectedPack = packId;
      this.save();
    },
    setFavoritesCollapsed(value: boolean) {
      this.favoritesCollapsed = value;
      this.save();
    },
    setPanelCollapsed(value: boolean) {
      this.panelCollapsed = value;
      this.save();
    },
    setDarkMode(mode: DarkMode) {
      this.darkMode = mode;
      if (mode === 'auto') {
        Dark.set('auto');
      } else {
        Dark.set(mode === 'dark');
      }
      this.save();
    },
    setLanguage(lang: Language) {
      this.language = lang;
      this.save();
    },
    setDebugPanelPos(pos: { x: number; y: number }) {
      this.debugPanelPos = pos;
      this.save();
    },
    addAcceptedStartupDialog(id: string) {
      if (!this.acceptedStartupDialogs.includes(id)) {
        this.acceptedStartupDialogs.push(id);
        this.save();
      }
    },
    setFavoritesOpensNewStack(value: boolean) {
      this.favoritesOpensNewStack = value;
      this.save();
    },
    setCompletedTutorial(value: boolean) {
      this.completedTutorial = value;
      this.save();
    },
    setCircuitCollectionPreviewShowPieces(value: boolean) {
      this.circuitCollectionPreviewShowPieces = value;
      this.save();
    },
    setCircuitEditorPiecePanel(value: CircuitEditorPiecePanelState) {
      this.circuitEditorPiecePanel = {
        x: value.x,
        y: value.y,
        width: value.width,
        height: value.height,
        minimized: value.minimized,
        docked: value.docked,
      };
      this.save();
    },
    save() {
      localStorage.setItem(
        'jei.settings',
        JSON.stringify({
          historyLimit: this.historyLimit,
          debugLayout: this.debugLayout,
          debugNavPanel: this.debugNavPanel,
          recipeViewMode: this.recipeViewMode,
          recipeSlotShowName: this.recipeSlotShowName,
          selectedPack: this.selectedPack,
          favoritesCollapsed: this.favoritesCollapsed,
          panelCollapsed: this.panelCollapsed,
          darkMode: this.darkMode,
          language: this.language,
          debugPanelPos: this.debugPanelPos,
          acceptedStartupDialogs: this.acceptedStartupDialogs,
          completedTutorial: this.completedTutorial,
          favoritesOpensNewStack: this.favoritesOpensNewStack,
          circuitCollectionPreviewShowPieces: this.circuitCollectionPreviewShowPieces,
          circuitEditorPiecePanel: this.circuitEditorPiecePanel,
        }),
      );
    },
  },
});
