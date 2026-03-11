<template>
  <Teleport to="body">
    <Transition name="viewer-fade">
      <div v-if="visible" class="image-viewer-overlay" :class="{ 'body--dark': isDark }" @click.self="close"
        @wheel.prevent="onWheel" @contextmenu.prevent="onDesktopContextMenu">
        <!-- 图片容器 -->
        <div ref="containerRef" class="viewer-container" @pointerdown="onPointerDown" @pointermove="onPointerMove"
          @pointerup="onPointerUp" @pointercancel="onPointerUp" @touchstart.passive="onTouchStart"
          @touchmove.prevent="onTouchMove" @touchend="onTouchEnd">
          <img ref="imageRef" :src="src" :alt="alt" class="viewer-image" :style="imageTransformStyle" draggable="false"
            @load="onImageLoad" />
        </div>

        <!-- 底部功能区 -->
        <div class="viewer-toolbar">
          <span class="viewer-title">{{ alt || t('imageViewer.title') }}</span>
          <div class="viewer-toolbar-actions">
            <button class="viewer-btn" :title="t('imageViewer.zoomOut')" @click="zoomOut">
              <q-icon name="remove" size="22px" />
            </button>
            <span class="viewer-zoom-label">{{ zoomPercent }}%</span>
            <button class="viewer-btn" :title="t('imageViewer.zoomIn')" @click="zoomIn">
              <q-icon name="add" size="22px" />
            </button>
            <button class="viewer-btn" :title="t('imageViewer.reset')" @click="resetTransform">
              <q-icon name="fit_screen" size="22px" />
            </button>
            <button class="viewer-btn" :title="t('close')" @click="close">
              <q-icon name="close" size="22px" />
            </button>
          </div>
        </div>

        <!-- 自定义右键/长按菜单 -->
        <Transition name="menu-fade">
          <div v-if="contextMenuVisible" class="viewer-context-menu" :style="contextMenuStyle" @click.stop>
            <template v-if="isMobile">
              <button class="context-menu-item" @click="saveImage">
                <q-icon name="save_alt" size="18px" />
                <span>{{ t('imageViewer.menuSaveImage') }}</span>
              </button>
              <button class="context-menu-item" @click="shareImage">
                <q-icon name="share" size="18px" />
                <span>{{ t('imageViewer.menuShare') }}</span>
              </button>
            </template>
            <template v-else>
              <button class="context-menu-item" @click="saveImage">
                <q-icon name="save_alt" size="18px" />
                <span>{{ t('imageViewer.menuSave') }}</span>
              </button>
              <button class="context-menu-item" @click="saveImageAs">
                <q-icon name="download" size="18px" />
                <span>{{ t('imageViewer.menuSaveAs') }}</span>
              </button>
              <button class="context-menu-item" @click="copyImage">
                <q-icon name="content_copy" size="18px" />
                <span>{{ t('imageViewer.menuCopy') }}</span>
              </button>
              <button class="context-menu-item" @click="shareImage">
                <q-icon name="share" size="18px" />
                <span>{{ t('imageViewer.menuShare') }}</span>
              </button>
            </template>
          </div>
        </Transition>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onBeforeUnmount, nextTick } from 'vue';
import { useQuasar } from 'quasar';
import { useI18n } from 'vue-i18n';
import {
  describeInvokeError,
  isMobileTauriRuntime,
  saveMediaFromUrl,
  saveMediaFromBlob,
} from 'src/utils/android-media';

const props = defineProps<{
  visible: boolean;
  src: string;
  alt?: string;
  originalUrl?: string;
}>();

const emit = defineEmits<{
  (e: 'update:visible', value: boolean): void;
}>();

const $q = useQuasar();
const { t } = useI18n();
const isDark = computed(() => $q.dark.isActive);
const isMobile = computed(() => $q.platform.is.mobile === true);

// --- Transform state ---
const scale = ref(1);
const translateX = ref(0);
const translateY = ref(0);
const MIN_SCALE = 0.1;
const MAX_SCALE = 20;
const ZOOM_STEP = 0.15;

const zoomPercent = computed(() => Math.round(scale.value * 100));

const imageTransformStyle = computed(() => ({
  transform: `translate(${translateX.value}px, ${translateY.value}px) scale(${scale.value})`,
  transition: isGesturing.value ? 'none' : 'transform 0.2s ease',
}));

// --- Refs ---
const containerRef = ref<HTMLElement | null>(null);
const imageRef = ref<HTMLImageElement | null>(null);

// --- Context menu ---
const contextMenuVisible = ref(false);
const contextMenuX = ref(0);
const contextMenuY = ref(0);
const contextMenuStyle = computed(() => {
  // 确保菜单不超出视窗
  const menuWidth = 180;
  const menuHeight = isMobile.value ? 100 : 180;
  const viewport = window.visualViewport;
  const vw = viewport?.width ?? window.innerWidth;
  const vh = viewport?.height ?? window.innerHeight;
  const offsetLeft = viewport?.offsetLeft ?? 0;
  const offsetTop = viewport?.offsetTop ?? 0;
  let x = contextMenuX.value;
  let y = contextMenuY.value;
  const rightBound = offsetLeft + vw;
  const bottomBound = offsetTop + vh;
  if (x + menuWidth > rightBound) x = rightBound - menuWidth - 8;
  if (y + menuHeight > bottomBound) y = bottomBound - menuHeight - 8;
  if (x < offsetLeft + 8) x = offsetLeft + 8;
  if (y < offsetTop + 8) y = offsetTop + 8;
  return { left: `${x}px`, top: `${y}px` };
});

// --- Long press for mobile ---
let longPressTimer: ReturnType<typeof setTimeout> | null = null;
const LONG_PRESS_DURATION = 500;

// --- Gesture state ---
const isGesturing = ref(false);
const isDragging = ref(false);
let dragStartX = 0;
let dragStartY = 0;
let dragStartTranslateX = 0;
let dragStartTranslateY = 0;

// Pinch zoom state
let initialPinchDistance = 0;
let initialPinchScale = 1;
let pinchCenterX = 0;
let pinchCenterY = 0;
const activeTouches = new Map<number, { x: number; y: number }>();

// ================== Zoom functions ==================

function zoomIn() {
  setScale(scale.value * (1 + ZOOM_STEP));
}

function zoomOut() {
  setScale(scale.value / (1 + ZOOM_STEP));
}

function setScale(newScale: number, centerX?: number, centerY?: number) {
  const clamped = Math.max(MIN_SCALE, Math.min(MAX_SCALE, newScale));
  if (centerX !== undefined && centerY !== undefined) {
    // Zoom towards pointer/pinch center
    const ratio = clamped / scale.value;
    translateX.value = centerX - ratio * (centerX - translateX.value);
    translateY.value = centerY - ratio * (centerY - translateY.value);
  }
  scale.value = clamped;
}

function resetTransform() {
  scale.value = 1;
  translateX.value = 0;
  translateY.value = 0;
}

// ================== Wheel zoom (desktop) ==================

function onWheel(e: WheelEvent) {
  closeContextMenu();
  const container = containerRef.value;
  if (!container) return;
  const rect = container.getBoundingClientRect();
  const cx = e.clientX - rect.left - rect.width / 2;
  const cy = e.clientY - rect.top - rect.height / 2;
  const delta = e.deltaY > 0 ? 1 / (1 + ZOOM_STEP) : 1 + ZOOM_STEP;
  setScale(scale.value * delta, cx, cy);
}

// ================== Pointer drag (desktop) ==================

function onPointerDown(e: PointerEvent) {
  // Only handle left button drag for desktop
  if (e.pointerType === 'touch') return;
  if (e.button !== 0) return;
  closeContextMenu();
  isDragging.value = true;
  isGesturing.value = true;
  dragStartX = e.clientX;
  dragStartY = e.clientY;
  dragStartTranslateX = translateX.value;
  dragStartTranslateY = translateY.value;
  (e.currentTarget as HTMLElement)?.setPointerCapture(e.pointerId);
}

function onPointerMove(e: PointerEvent) {
  if (e.pointerType === 'touch') return;
  if (!isDragging.value) return;
  translateX.value = dragStartTranslateX + (e.clientX - dragStartX);
  translateY.value = dragStartTranslateY + (e.clientY - dragStartY);
}

function onPointerUp(e: PointerEvent) {
  if (e.pointerType === 'touch') return;
  isDragging.value = false;
  isGesturing.value = false;
}

// ================== Touch gestures (mobile) ==================

function onTouchStart(e: TouchEvent) {
  closeContextMenu();

  for (let i = 0; i < e.changedTouches.length; i++) {
    const t = e.changedTouches.item(i);
    if (!t) continue;
    activeTouches.set(t.identifier, { x: t.clientX, y: t.clientY });
  }

  if (activeTouches.size === 1) {
    // Single finger: start drag + long press detection
    const t = e.touches.item(0);
    if (!t) return;
    isDragging.value = true;
    isGesturing.value = true;
    dragStartX = t.clientX;
    dragStartY = t.clientY;
    dragStartTranslateX = translateX.value;
    dragStartTranslateY = translateY.value;

    const tx = t.clientX;
    const ty = t.clientY;
    longPressTimer = setTimeout(() => {
      contextMenuX.value = tx;
      contextMenuY.value = ty;
      contextMenuVisible.value = true;
      isDragging.value = false;
    }, LONG_PRESS_DURATION);
  }

  if (activeTouches.size === 2) {
    // Two fingers: pinch zoom
    cancelLongPress();
    isDragging.value = false;
    const touches = Array.from(activeTouches.values());
    const t0 = touches[0];
    const t1 = touches[1];
    if (!t0 || !t1) return;
    initialPinchDistance = getDistance(t0, t1);
    initialPinchScale = scale.value;
    pinchCenterX = (t0.x + t1.x) / 2;
    pinchCenterY = (t0.y + t1.y) / 2;
    // Convert to container-relative coords
    const container = containerRef.value;
    if (container) {
      const rect = container.getBoundingClientRect();
      pinchCenterX -= rect.left + rect.width / 2;
      pinchCenterY -= rect.top + rect.height / 2;
    }
  }
}

function onTouchMove(e: TouchEvent) {
  for (let i = 0; i < e.changedTouches.length; i++) {
    const t = e.changedTouches.item(i);
    if (!t) continue;
    activeTouches.set(t.identifier, { x: t.clientX, y: t.clientY });
  }

  if (activeTouches.size === 1 && isDragging.value) {
    // Single finger drag
    const t = e.touches.item(0);
    if (!t) return;
    const dx = t.clientX - dragStartX;
    const dy = t.clientY - dragStartY;
    // Cancel long press if moved significantly
    if (Math.abs(dx) > 10 || Math.abs(dy) > 10) {
      cancelLongPress();
    }
    translateX.value = dragStartTranslateX + dx;
    translateY.value = dragStartTranslateY + dy;
  }

  if (activeTouches.size === 2) {
    // Pinch zoom
    const touches = Array.from(activeTouches.values());
    const t0 = touches[0];
    const t1 = touches[1];
    if (!t0 || !t1) return;
    const dist = getDistance(t0, t1);
    const newScale = initialPinchScale * (dist / initialPinchDistance);
    setScale(newScale, pinchCenterX, pinchCenterY);
  }
}

function onTouchEnd(e: TouchEvent) {
  cancelLongPress();
  for (let i = 0; i < e.changedTouches.length; i++) {
    const t = e.changedTouches.item(i);
    if (t) activeTouches.delete(t.identifier);
  }
  if (activeTouches.size === 0) {
    isDragging.value = false;
    isGesturing.value = false;
  }
}

function getDistance(a: { x: number; y: number }, b: { x: number; y: number }) {
  return Math.hypot(a.x - b.x, a.y - b.y);
}

function cancelLongPress() {
  if (longPressTimer) {
    clearTimeout(longPressTimer);
    longPressTimer = null;
  }
}

// ================== Context menu (desktop right-click) ==================

function onDesktopContextMenu(e: MouseEvent) {
  if (isMobile.value) return;
  contextMenuX.value = e.clientX;
  contextMenuY.value = e.clientY;
  contextMenuVisible.value = true;
}

function closeContextMenu() {
  contextMenuVisible.value = false;
}

// ================== Image operations ==================

const httpUrlPattern = /^https?:\/\//i;
const protocolUrlPattern = /^(asset|tauri|file):/i;
const absolutePathPattern = /^([a-zA-Z]:\\|\\\\|\/)/;

function guessMimeType(filename: string): string {
  const ext = filename.split('.').pop()?.toLowerCase() || '';
  const map: Record<string, string> = {
    png: 'image/png',
    jpg: 'image/jpeg',
    jpeg: 'image/jpeg',
    webp: 'image/webp',
    gif: 'image/gif',
    svg: 'image/svg+xml',
    avif: 'image/avif',
    bmp: 'image/bmp',
    ico: 'image/x-icon',
  };
  return map[ext] || 'image/png';
}

/**
 * 通过 Tauri plugin-fs 直接读取本地文件
 */
async function readTauriLocalFile(url: string): Promise<Blob | null> {
  try {
    const { readFile, BaseDirectory } = await import('@tauri-apps/plugin-fs');
    let bytes: Uint8Array;
    if (absolutePathPattern.test(url)) {
      // 绝对路径 — 直接读取（需要 FS scope 允许）
      bytes = await readFile(url);
    } else {
      // 相对路径 — 从 AppData 读取
      const relativePath = url.startsWith('/') ? url.slice(1) : url;
      bytes = await readFile(relativePath, { baseDir: BaseDirectory.AppData });
    }
    const mimeType = guessMimeType(url);
    return new Blob([bytes.buffer as ArrayBuffer], { type: mimeType });
  } catch {
    return null;
  }
}

/**
 * 通过 Tauri plugin-http 获取远程图片（可绕过 CORS 限制）
 */
async function fetchViaTauriHttp(url: string): Promise<Blob | null> {
  try {
    const { fetch: tauriFetch } = await import('@tauri-apps/plugin-http');
    const response = await tauriFetch(url);
    if (!response.ok) return null;
    return await response.blob();
  } catch {
    return null;
  }
}

/**
 * 获取图片 Blob 的统一入口
 * 策略优先级：
 * 1. Tauri 本地文件 → plugin-fs readFile
 * 2. 标准 fetch (cors) → 标准 fetch (no-cors fallback)
 * 3. Tauri HTTP plugin fetch（绕过 CORS）
 * 4. Canvas fallback（从已加载的 img 元素导出）
 */
async function fetchImageBlob(): Promise<Blob | null> {
  const isTauri = !!window.__TAURI_INTERNALS__;
  const rawUrl = props.originalUrl || props.src;

  // 1) Tauri 本地文件：直接通过 plugin-fs 读取
  if (isTauri && !httpUrlPattern.test(rawUrl) && !protocolUrlPattern.test(rawUrl)) {
    const localBlob = await readTauriLocalFile(rawUrl);
    if (localBlob) return localBlob;
  }

  // 2) 标准 fetch（适用于 HTTP URL 或 Web 环境）
  const fetchUrl = httpUrlPattern.test(props.src) ? props.src : rawUrl;
  if (httpUrlPattern.test(fetchUrl)) {
    try {
      const response = await fetch(fetchUrl, { mode: 'cors' });
      if (response.ok) return await response.blob();
    } catch { /* CORS blocked — continue */ }

    // 3) Tauri HTTP plugin fetch（绕过 CORS）
    if (isTauri) {
      const tauriBlob = await fetchViaTauriHttp(fetchUrl);
      if (tauriBlob) return tauriBlob;
    }

    try {
      const response = await fetch(fetchUrl);
      if (response.ok) return await response.blob();
    } catch { /* continue */ }
  }

  // 4) Canvas fallback（从 viewer 中已显示的 img 获取）
  if (imageRef.value?.complete && imageRef.value.naturalWidth > 0) {
    return imageElementToBlob(imageRef.value);
  }

  return null;
}

function imageElementToBlob(img: HTMLImageElement): Promise<Blob | null> {
  return new Promise((resolve) => {
    try {
      const width = img.naturalWidth;
      const height = img.naturalHeight;
      if (!width || !height) {
        resolve(null);
        return;
      }
      const canvas = document.createElement('canvas');
      canvas.width = width;
      canvas.height = height;
      const context = canvas.getContext('2d');
      if (!context) {
        resolve(null);
        return;
      }
      context.drawImage(img, 0, 0, width, height);
      canvas.toBlob((blob) => resolve(blob), 'image/png');
    } catch {
      resolve(null);
    }
  });
}

function getFilenameFromUrl(url: string): string {
  try {
    const pathname = new URL(url).pathname;
    const segments = pathname.split('/');
    const last = segments[segments.length - 1];
    if (last && last.includes('.')) return decodeURIComponent(last);
  } catch {
    // ignore
  }
  return 'image.png';
}

function triggerBrowserDownload(url: string, filename?: string) {
  const a = document.createElement('a');
  a.href = url;
  if (filename) {
    a.download = filename;
  }
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
}

function sanitizeFileName(name: string) {
  return name.replace(/[\\/:*?"<>|]/g, '_');
}

function ensureImageFilename(name: string, mimeType: string) {
  if (name.includes('.')) return sanitizeFileName(name);
  const mimeToExt: Record<string, string> = {
    'image/png': 'png',
    'image/jpeg': 'jpg',
    'image/webp': 'webp',
    'image/gif': 'gif',
    'image/svg+xml': 'svg',
    'image/avif': 'avif',
  };
  const ext = mimeToExt[mimeType] || 'png';
  return sanitizeFileName(`${name}.${ext}`);
}

/**
 * Desktop-only fallback: write to Downloads via plugin-fs.
 * NOT used on mobile — on Android, BaseDirectory.Download resolves to app-private directory,
 * not the public Downloads folder. Mobile must always use the native MediaStore plugin.
 */
async function saveBlobToTauriDownloads(blob: Blob, fileName: string) {
  const { writeFile, BaseDirectory } = await import('@tauri-apps/plugin-fs');
  const bytes = new Uint8Array(await blob.arrayBuffer());
  await writeFile(fileName, bytes, { baseDir: BaseDirectory.Download });
}

async function saveImage() {
  closeContextMenu();
  const isTauri = !!window.__TAURI_INTERNALS__;
  try {
    const rawName = getFilenameFromUrl(props.originalUrl || props.src);

    if (isTauri) {
      // Determine the original CDN URL (prefer originalUrl which is the raw CDN source)
      const cdnUrl = props.originalUrl || props.src;
      const isHttpUrl = httpUrlPattern.test(cdnUrl);

      if (isHttpUrl) {
        // Primary path: Rust backend downloads directly from CDN
        try {
          const mimeType = guessMimeType(rawName);
          const fileName = ensureImageFilename(rawName, mimeType);
          await saveMediaFromUrl(cdnUrl, fileName, mimeType, 'images');
          $q.notify({ type: 'positive', message: t('imageViewer.savedToAlbum'), timeout: 1800 });
          return;
        } catch (error: unknown) {
          // If Rust download fails, fall through to blob fallback
          console.warn('Rust download_and_save_media failed, trying blob fallback:', error);
        }
      }

      // Fallback: fetch blob in frontend, send base64 to Rust backend
      const blob = await fetchImageBlob();
      if (!blob) {
        $q.notify({ type: 'negative', message: t('imageViewer.downloadFailed') });
        return;
      }
      const fileName = ensureImageFilename(rawName, blob.type || 'image/png');
      try {
        await saveMediaFromBlob(blob, fileName, blob.type || 'image/png', 'images');
        $q.notify({ type: 'positive', message: t('imageViewer.savedToAlbum'), timeout: 1800 });
      } catch (error: unknown) {
        // Desktop-only last resort: plugin-fs writes to ~/Downloads.
        // NEVER use on mobile — BaseDirectory.Download resolves to app-private directory.
        if (!isMobileTauriRuntime()) {
          try {
            await saveBlobToTauriDownloads(blob, fileName);
            $q.notify({
              type: 'positive',
              message: t('imageViewer.savedToDownloads'),
              timeout: 2200,
            });
            return;
          } catch { /* fall through to error */ }
        }
        const detail = describeInvokeError(error);
        $q.notify({ type: 'negative', message: t('imageViewer.saveFailed'), caption: detail, timeout: 2600 });
      }
      return;
    }

    // Web browser: fetch blob and trigger download
    const blob = await fetchImageBlob();
    if (!blob) {
      $q.notify({ type: 'negative', message: t('imageViewer.downloadFailed') });
      return;
    }
    const fileName = ensureImageFilename(rawName, blob.type || 'image/png');
    const url = URL.createObjectURL(blob);
    triggerBrowserDownload(url, fileName);
    URL.revokeObjectURL(url);
    $q.notify({ type: 'positive', message: t('imageViewer.downloadStarted'), timeout: 1500 });
  } catch {
    $q.notify({ type: 'negative', message: t('imageViewer.saveFailed') });
  }
}

async function saveImageAs() {
  // 桌面端"另存为"使用 showSaveFilePicker API（如果可用），否则退回到普通保存
  closeContextMenu();
  if ('showSaveFilePicker' in window) {
    try {
      const blob = await fetchImageBlob();
      if (!blob) {
        $q.notify({ type: 'negative', message: t('imageViewer.downloadFailed') });
        return;
      }
      const ext = getFilenameFromUrl(props.originalUrl || props.src).split('.').pop() || 'png';
      const mimeType = blob.type || 'image/png';
      const handle = await (window as unknown as { showSaveFilePicker: (opts: unknown) => Promise<FileSystemFileHandle> }).showSaveFilePicker({
        suggestedName: getFilenameFromUrl(props.originalUrl || props.src),
        types: [
          {
            description: t('imageViewer.fileTypeImage'),
            accept: { [mimeType]: [`.${ext}`] },
          },
        ],
      });
      const writable = await handle.createWritable();
      await writable.write(blob);
      await writable.close();
      $q.notify({ type: 'positive', message: t('imageViewer.saved'), timeout: 1500 });
    } catch (err: unknown) {
      // 用户取消不提示
      if (err instanceof DOMException && err.name === 'AbortError') return;
      $q.notify({ type: 'negative', message: t('imageViewer.saveFailed') });
    }
  } else {
    await saveImage();
  }
}

async function copyImage() {
  closeContextMenu();
  try {
    const blob = await fetchImageBlob();
    if (!blob) {
      $q.notify({ type: 'negative', message: t('imageViewer.fetchFailed') });
      return;
    }
    // ClipboardItem 需要 image/png 格式
    let pngBlob = blob;
    if (blob.type !== 'image/png') {
      // 转换为 PNG
      pngBlob = await convertToPng(blob);
    }
    await navigator.clipboard.write([
      new ClipboardItem({ 'image/png': pngBlob }),
    ]);
    $q.notify({ type: 'positive', message: t('imageViewer.copiedImage'), timeout: 1500 });
  } catch {
    $q.notify({ type: 'negative', message: t('imageViewer.copyFailed') });
  }
}

function convertToPng(blob: Blob): Promise<Blob> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.crossOrigin = 'anonymous';
    img.onload = () => {
      const canvas = document.createElement('canvas');
      canvas.width = img.naturalWidth;
      canvas.height = img.naturalHeight;
      const ctx = canvas.getContext('2d');
      if (!ctx) {
        reject(new Error('Canvas context unavailable'));
        return;
      }
      ctx.drawImage(img, 0, 0);
      canvas.toBlob(
        (result) => {
          if (result) resolve(result);
          else reject(new Error('toBlob failed'));
        },
        'image/png',
      );
    };
    img.onerror = () => reject(new Error('Image load failed'));
    img.src = URL.createObjectURL(blob);
  });
}

async function shareImage() {
  closeContextMenu();
  try {
    if (navigator.share) {
      const blob = await fetchImageBlob();
      if (blob) {
        const filename = getFilenameFromUrl(props.src);
        const file = new File([blob], filename, { type: blob.type });
        await navigator.share({
          title: props.alt || t('imageViewer.shareTitle'),
          files: [file],
        });
        return;
      }
      // Fallback: share URL
      await navigator.share({
        title: props.alt || t('imageViewer.shareTitle'),
        url: props.src,
      });
    } else {
      // 不支持 Web Share API，复制链接
      await navigator.clipboard.writeText(props.src);
      $q.notify({ type: 'positive', message: t('imageViewer.copiedLink'), timeout: 1500 });
    }
  } catch (err: unknown) {
    if (err instanceof DOMException && err.name === 'AbortError') return;
    $q.notify({ type: 'negative', message: t('imageViewer.shareFailed') });
  }
}

// ================== Image load ==================

function onImageLoad() {
  // 加载后自适应缩放
  if (!imageRef.value || !containerRef.value) return;
  const img = imageRef.value;
  const container = containerRef.value;
  const cw = container.clientWidth * 0.9;
  const ch = container.clientHeight * 0.9;
  const iw = img.naturalWidth;
  const ih = img.naturalHeight;
  if (iw > cw || ih > ch) {
    scale.value = Math.min(cw / iw, ch / ih);
  } else {
    scale.value = 1;
  }
  translateX.value = 0;
  translateY.value = 0;
}

// ================== Open/Close ==================

function close() {
  closeContextMenu();
  cancelLongPress();
  emit('update:visible', false);
}

// ESC key to close
function onKeyDown(e: KeyboardEvent) {
  if (!props.visible) return;
  if (e.key === 'Escape') {
    close();
  } else if (e.key === '+' || e.key === '=') {
    zoomIn();
  } else if (e.key === '-') {
    zoomOut();
  } else if (e.key === '0') {
    resetTransform();
  }
}

// Click outside context menu to close it
function onGlobalClick() {
  if (contextMenuVisible.value) {
    closeContextMenu();
  }
}

watch(
  () => props.visible,
  (val) => {
    if (val) {
      resetTransform();
      void nextTick(() => {
        onImageLoad();
      });
      // Prevent body scroll
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = '';
    }
  },
);

onMounted(() => {
  window.addEventListener('keydown', onKeyDown);
  window.addEventListener('click', onGlobalClick);
});

onBeforeUnmount(() => {
  window.removeEventListener('keydown', onKeyDown);
  window.removeEventListener('click', onGlobalClick);
  document.body.style.overflow = '';
  cancelLongPress();
});
</script>

<style scoped lang="scss">
.image-viewer-overlay {
  position: fixed;
  inset: 0;
  z-index: 6000;
  display: flex;
  flex-direction: column;
  background: rgba(0, 0, 0, 0.92);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  user-select: none;
  -webkit-user-select: none;
  touch-action: none;
}

// ==================== Toolbar ====================
.viewer-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  height: calc(48px + env(safe-area-inset-bottom));
  min-height: calc(48px + env(safe-area-inset-bottom));
  padding-bottom: env(safe-area-inset-bottom);
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  box-shadow: 0 -4px 16px rgba(0, 0, 0, 0.35);
  z-index: 2;
}

.viewer-title {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.85);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 40%;
}

.viewer-toolbar-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.viewer-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border: none;
  border-radius: 8px;
  background: rgba(255, 255, 255, 0.08);
  color: rgba(255, 255, 255, 0.85);
  cursor: pointer;
  transition: background 0.15s, color 0.15s;

  &:hover {
    background: rgba(255, 255, 255, 0.18);
    color: #fff;
  }

  &:active {
    background: rgba(255, 255, 255, 0.25);
  }
}

.viewer-zoom-label {
  font-size: 12px;
  color: rgba(255, 255, 255, 0.65);
  width: 42px;
  text-align: center;
  font-variant-numeric: tabular-nums;
}

// ==================== Container ====================
.viewer-container {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  cursor: grab;

  &:active {
    cursor: grabbing;
  }
}

.viewer-image {
  max-width: none;
  max-height: none;
  transform-origin: center center;
  border-radius: 4px;
  box-shadow: 0 4px 24px rgba(0, 0, 0, 0.4);
  pointer-events: none;
}

// ==================== Context Menu ====================
.viewer-context-menu {
  position: fixed;
  z-index: 6002;
  min-width: 160px;
  padding: 6px 0;
  background: rgba(30, 30, 30, 0.96);
  backdrop-filter: blur(16px);
  -webkit-backdrop-filter: blur(16px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  box-shadow:
    0 8px 32px rgba(0, 0, 0, 0.5),
    0 2px 8px rgba(0, 0, 0, 0.3);
  overflow: hidden;

  .body--dark & {
    background: rgba(30, 30, 30, 0.96);
  }
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 10px 16px;
  border: none;
  background: transparent;
  color: rgba(255, 255, 255, 0.85);
  font-size: 14px;
  cursor: pointer;
  transition: background 0.12s;
  text-align: left;

  &:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  &:active {
    background: rgba(255, 255, 255, 0.18);
  }

  .q-icon {
    color: rgba(255, 255, 255, 0.65);
  }
}

// ==================== Transitions ====================
.viewer-fade-enter-active {
  transition: opacity 0.25s ease;
}

.viewer-fade-leave-active {
  transition: opacity 0.2s ease;
}

.viewer-fade-enter-from,
.viewer-fade-leave-to {
  opacity: 0;
}

.menu-fade-enter-active {
  transition:
    opacity 0.15s ease,
    transform 0.15s ease;
}

.menu-fade-leave-active {
  transition:
    opacity 0.1s ease,
    transform 0.1s ease;
}

.menu-fade-enter-from {
  opacity: 0;
  transform: scale(0.92);
}

.menu-fade-leave-to {
  opacity: 0;
  transform: scale(0.95);
}

// ==================== Responsive ====================
@media (max-width: 600px) {
  .viewer-toolbar {
    padding: 0 12px;
    height: calc(44px + env(safe-area-inset-bottom));
    min-height: calc(44px + env(safe-area-inset-bottom));
  }

  .viewer-title {
    font-size: 13px;
    max-width: 35%;
  }

  .viewer-btn {
    width: 32px;
    height: 32px;
  }

  .viewer-zoom-label {
    font-size: 11px;
    width: 36px;
  }

  .viewer-context-menu {
    border-radius: 16px;
    min-width: 180px;
  }

  .context-menu-item {
    padding: 14px 18px;
    font-size: 15px;
  }
}
</style>
