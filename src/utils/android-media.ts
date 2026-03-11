/**
 * Media save utilities — platform-split architecture.
 *
 * Mobile (Android / iOS):
 *   Native plugin handles EVERYTHING in one step — download + save to
 *   MediaStore (Android) or Photos (iOS). This avoids Rust HTTP on mobile
 *   where reqwest + rustls can't use the system certificate store / DNS.
 *
 * Desktop (Windows / macOS / Linux):
 *   Rust backend downloads via reqwest and saves directly to ~/Downloads.
 */

import { invoke } from '@tauri-apps/api/core';

export type MediaTarget = 'images' | 'downloads';

interface RustSaveResult {
  success: boolean;
  path: string;
  message: string;
}

function isTauriRuntime(): boolean {
  return typeof window !== 'undefined' && window.__TAURI_INTERNALS__ !== undefined;
}

export function isAndroidTauriRuntime(): boolean {
  if (!isTauriRuntime()) return false;
  if (typeof navigator === 'undefined') return false;
  return /android/i.test(navigator.userAgent);
}

export function isIOSTauriRuntime(): boolean {
  if (!isTauriRuntime()) return false;
  if (typeof navigator === 'undefined') return false;
  return /iphone|ipad|ipod/i.test(navigator.userAgent);
}

export function isMobileTauriRuntime(): boolean {
  return isAndroidTauriRuntime() || isIOSTauriRuntime();
}

export function describeInvokeError(error: unknown): string {
  if (error instanceof Error) return error.message;
  if (typeof error === 'string') return error;
  if (error && typeof error === 'object') {
    const record = error as Record<string, unknown>;
    const firstMessage = [
      record.message,
      record.error,
      record.reason,
      record.details,
      record.cause,
    ].find((value) => typeof value === 'string' && value.trim().length > 0);
    if (typeof firstMessage === 'string') return firstMessage;
    try {
      return JSON.stringify(error);
    } catch {
      return Object.prototype.toString.call(error);
    }
  }
  return String(error);
}

function sanitizeFileName(fileName: string): string {
  const normalized = fileName.trim().replace(/[\\/:*?"<>|]/g, '_');
  return normalized || `file-${Date.now()}`;
}

function blobToBase64(blob: Blob): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => {
      const dataUrl = typeof reader.result === 'string' ? reader.result : '';
      const base64 = dataUrl.split(',')[1] || '';
      if (!base64) {
        reject(new Error('Failed to convert blob to base64'));
        return;
      }
      resolve(base64);
    };
    reader.onerror = () => reject(new Error('Failed to read blob'));
    reader.readAsDataURL(blob);
  });
}

/**
 * Save media from a CDN / HTTP URL.
 *
 * Mobile: native plugin downloads + saves in one step (HttpURLConnection +
 *         MediaStore on Android, URLSession + Photos on iOS).
 * Desktop: Rust `download_and_save_media` → ~/Downloads.
 */
export async function saveMediaFromUrl(
  url: string,
  fileName: string,
  mimeType: string,
  target: MediaTarget,
): Promise<void> {
  if (!isTauriRuntime()) {
    throw new Error('Tauri runtime is required');
  }

  const safeName = sanitizeFileName(fileName);

  if (isMobileTauriRuntime()) {
    // ── Mobile: native plugin (single step) ──
    await invoke('plugin:android-intent|saveMediaFromUrl', {
      sourceUrl: url,
      fileName: safeName,
      mimeType,
      target,
    });
    return;
  }

  // ── Desktop: Rust backend ──
  const result = await invoke<RustSaveResult>('download_and_save_media', {
    url,
    fileName: safeName,
    mimeType,
    target,
  });

  if (!result.success) {
    throw new Error(result.message || 'Rust download_and_save_media failed');
  }
}

/**
 * Save media from a Blob (non-HTTP sources: canvas, local data, etc.).
 *
 * Mobile: converts to base64 → native plugin `saveMedia` writes to MediaStore / Photos.
 * Desktop: converts to base64 → Rust `save_media_from_bytes` → ~/Downloads.
 */
export async function saveMediaFromBlob(
  blob: Blob,
  fileName: string,
  mimeType: string,
  target: MediaTarget,
): Promise<void> {
  if (!isTauriRuntime()) {
    throw new Error('Tauri runtime is required');
  }

  const base64Data = await blobToBase64(blob);
  const safeName = sanitizeFileName(fileName);

  if (isMobileTauriRuntime()) {
    // ── Mobile: native plugin (single step) ──
    await invoke('plugin:android-intent|saveMedia', {
      base64Data,
      fileName: safeName,
      mimeType,
      target,
    });
    return;
  }

  // ── Desktop: Rust backend ──
  const result = await invoke<RustSaveResult>('save_media_from_bytes', {
    base64Data,
    fileName: safeName,
    mimeType,
    target,
  });

  if (!result.success) {
    throw new Error(result.message || 'Rust save_media_from_bytes failed');
  }
}
