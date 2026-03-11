import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export type DialogPriority = 'low' | 'medium' | 'high' | 'critical';
export type DialogStatus = 'pending' | 'showing' | 'completed' | 'skipped';

export interface DialogConfig {
  id: string;
  priority: DialogPriority;
  title: string;
  // 弹窗显示条件函数，返回true表示可以显示
  canShow?: () => boolean | Promise<boolean>;
  // 弹窗显示回调
  onShow?: () => void;
  // 弹窗关闭回调
  onClose?: () => void;
  // 是否需要用户操作才关闭（persistent）
  persistent?: boolean;
  // 自定义数据
  data?: Record<string, unknown>;
}

const PRIORITY_ORDER: Record<DialogPriority, number> = {
  critical: 4,
  high: 3,
  medium: 2,
  low: 1,
};

// 教程弹窗的特殊ID
export const TUTORIAL_DIALOG_ID = '__tutorial__';

export const useDialogManager = defineStore('dialogManager', () => {
  // 所有已注册的弹窗配置
  const registeredDialogs = ref<Map<string, DialogConfig>>(new Map());

  // 弹窗队列（按优先级排序）
  const dialogQueue = ref<string[]>([]);

  // 当前弹窗状态
  const dialogStatus = ref<Map<string, DialogStatus>>(new Map());

  // 当前显示的弹窗
  const currentDialogId = ref<string | null>(null);

  // 是否正在处理弹窗队列
  const isProcessingQueue = ref(false);

  // 获取当前显示的弹窗
  const currentDialog = computed(() => {
    if (!currentDialogId.value) return null;
    return registeredDialogs.value.get(currentDialogId.value) ?? null;
  });

  // 检查是否有弹窗正在显示
  const hasActiveDialog = computed(() => currentDialogId.value !== null);

  // 检查所有弹窗是否都已完成
  const allDialogsCompleted = computed(() => {
    const dialogs = registeredDialogs.value;
    for (const [id] of dialogs) {
      // 跳过教程弹窗
      if (id === TUTORIAL_DIALOG_ID) continue;
      const status = dialogStatus.value.get(id);
      if (status !== 'completed' && status !== 'skipped') {
        return false;
      }
    }
    return true;
  });

  // 注册弹窗配置
  function registerDialog(config: DialogConfig): void {
    registeredDialogs.value.set(config.id, config);
    dialogStatus.value.set(config.id, 'pending');
    updateQueue();
  }

  // 取消注册弹窗
  function unregisterDialog(id: string): void {
    registeredDialogs.value.delete(id);
    dialogStatus.value.delete(id);
    updateQueue();
  }

  // 更新弹窗队列（按优先级排序）
  function updateQueue(): void {
    const dialogs = Array.from(registeredDialogs.value.values());
    // 过滤出待处理的弹窗，按优先级排序
    dialogQueue.value = dialogs
      .filter((d) => {
        // 教程弹窗由 startTutorial 统一触发，不进入队列
        if (d.id === TUTORIAL_DIALOG_ID) return false;
        const status = dialogStatus.value.get(d.id);
        return status === 'pending';
      })
      .sort((a, b) => PRIORITY_ORDER[b.priority] - PRIORITY_ORDER[a.priority])
      .map((d) => d.id);
  }

  // 处理弹窗队列
  async function processQueue(): Promise<void> {
    if (isProcessingQueue.value) return;
    if (currentDialogId.value) return; // 当前有弹窗在显示

    isProcessingQueue.value = true;
    console.log('[dialogManager] processQueue start', {
      queue: [...dialogQueue.value],
      current: currentDialogId.value,
    });

    try {
      while (dialogQueue.value.length > 0) {
        const nextId = dialogQueue.value[0];
        if (!nextId) {
          dialogQueue.value.shift();
          continue;
        }

        const config = registeredDialogs.value.get(nextId);
        if (!config) {
          dialogQueue.value.shift();
          continue;
        }

        // 检查是否可以显示
        if (config.canShow) {
          const canShow = await config.canShow();
          if (!canShow) {
            // 不能显示，标记为跳过
            dialogStatus.value.set(nextId, 'skipped');
            dialogQueue.value.shift();
            updateQueue();
            continue;
          }
        }

        // 显示弹窗
        currentDialogId.value = nextId;
        dialogStatus.value.set(nextId, 'showing');
        console.log('[dialogManager] show dialog', nextId);
        config.onShow?.();

        // 等待弹窗关闭（由外部调用 completeDialog 或 skipDialog）
        await new Promise<void>((resolve) => {
          const checkInterval = setInterval(() => {
            if (currentDialogId.value !== nextId) {
              clearInterval(checkInterval);
              resolve();
            }
          }, 100);
        });

        // 弹窗关闭后继续下一个
        dialogQueue.value.shift();
      }

      // 队列处理完成，检查是否需要启动教程
      if (allDialogsCompleted.value) {
        console.log('[dialogManager] all dialogs completed, start tutorial');
        void startTutorial();
      }
    } finally {
      isProcessingQueue.value = false;
      console.log('[dialogManager] processQueue end', {
        queue: [...dialogQueue.value],
        current: currentDialogId.value,
      });
    }
  }

  // 完成当前弹窗
  function completeDialog(): void {
    const id = currentDialogId.value;
    if (id) {
      console.log('[dialogManager] complete dialog', id);
      dialogStatus.value.set(id, 'completed');
      const config = registeredDialogs.value.get(id);
      config?.onClose?.();
      currentDialogId.value = null;
      // 继续处理队列
      void processQueue();
    }
  }

  // 跳过当前弹窗
  function skipDialog(): void {
    const id = currentDialogId.value;
    if (id) {
      console.log('[dialogManager] skip dialog', id);
      dialogStatus.value.set(id, 'skipped');
      const config = registeredDialogs.value.get(id);
      config?.onClose?.();
      currentDialogId.value = null;
      // 继续处理队列
      void processQueue();
    }
  }

  // 手动触发队列处理
  function triggerProcess(): void {
    void processQueue();
  }

  // 获取弹窗状态
  function getDialogStatus(id: string): DialogStatus | undefined {
    return dialogStatus.value.get(id);
  }

  // 检查弹窗是否已完成或跳过
  function isDialogCompleted(id: string): boolean {
    const status = dialogStatus.value.get(id);
    return status === 'completed' || status === 'skipped';
  }

  // 检查弹窗是否正在显示
  function isDialogShowing(id: string): boolean {
    return currentDialogId.value === id;
  }

  // 重置单个弹窗状态为待处理
  function resetDialogStatus(id: string): void {
    if (!registeredDialogs.value.has(id)) return;
    dialogStatus.value.set(id, 'pending');
    updateQueue();
  }

  // 启动教程（由弹窗管理器在所有弹窗完成后调用）
  async function startTutorial(force = false): Promise<void> {
    if (currentDialogId.value) return;
    const status = dialogStatus.value.get(TUTORIAL_DIALOG_ID);
    if (status === 'completed' || status === 'skipped') {
      if (!force) return;
      dialogStatus.value.set(TUTORIAL_DIALOG_ID, 'pending');
    }

    console.log('[dialogManager] start tutorial', {
      force,
      status: dialogStatus.value.get(TUTORIAL_DIALOG_ID),
    });

    const tutorialConfig = registeredDialogs.value.get(TUTORIAL_DIALOG_ID);
    if (!tutorialConfig) return;

    if (tutorialConfig.canShow) {
      const canShow = await tutorialConfig.canShow();
      if (!canShow) {
        console.log('[dialogManager] tutorial canShow=false, skip');
        dialogStatus.value.set(TUTORIAL_DIALOG_ID, 'skipped');
        return;
      }
    }

    currentDialogId.value = TUTORIAL_DIALOG_ID;
    dialogStatus.value.set(TUTORIAL_DIALOG_ID, 'showing');
    tutorialConfig.onShow?.();
  }

  // 完成教程
  function completeTutorial(): void {
    if (currentDialogId.value === TUTORIAL_DIALOG_ID) {
      dialogStatus.value.set(TUTORIAL_DIALOG_ID, 'completed');
      const config = registeredDialogs.value.get(TUTORIAL_DIALOG_ID);
      config?.onClose?.();
      currentDialogId.value = null;
    }
  }

  // 跳过教程
  function skipTutorial(): void {
    if (currentDialogId.value === TUTORIAL_DIALOG_ID) {
      dialogStatus.value.set(TUTORIAL_DIALOG_ID, 'skipped');
      const config = registeredDialogs.value.get(TUTORIAL_DIALOG_ID);
      config?.onClose?.();
      currentDialogId.value = null;
    }
  }

  // 重置所有弹窗状态（用于测试）
  function resetAll(): void {
    dialogStatus.value.clear();
    currentDialogId.value = null;
    isProcessingQueue.value = false;
    dialogQueue.value = [];
    // 重新初始化所有弹窗为pending状态
    for (const id of registeredDialogs.value.keys()) {
      dialogStatus.value.set(id, 'pending');
    }
    updateQueue();
  }

  return {
    registeredDialogs,
    dialogQueue,
    currentDialog,
    currentDialogId,
    hasActiveDialog,
    allDialogsCompleted,
    registerDialog,
    unregisterDialog,
    completeDialog,
    skipDialog,
    triggerProcess,
    getDialogStatus,
    isDialogCompleted,
    isDialogShowing,
    resetDialogStatus,
    startTutorial,
    completeTutorial,
    skipTutorial,
    resetAll,
  };
});
