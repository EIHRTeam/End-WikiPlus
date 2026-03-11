import type { Router } from 'vue-router';

function canNavigateBack(router: Router): boolean {
  const stateBack = router.options.history.state.back;
  if (typeof stateBack === 'string' && stateBack.length > 0) {
    return true;
  }

  if (typeof window !== 'undefined') {
    return window.history.length > 1;
  }

  return false;
}

export async function backOrFallback(router: Router, fallbackPath = '/'): Promise<void> {
  if (canNavigateBack(router)) {
    router.back();
    return;
  }

  await router.push(fallbackPath).catch(() => undefined);
}
