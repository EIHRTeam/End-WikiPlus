import { convertFileSrc } from '@tauri-apps/api/core';
import { appDataDir, join } from '@tauri-apps/api/path';
import { readFile, BaseDirectory, exists } from '@tauri-apps/plugin-fs';

const IS_TAURI = window.__TAURI_INTERNALS__ !== undefined;
const httpUrlPattern = /^https?:\/\//i;
const protocolUrlPattern = /^(asset|tauri|file):/i;
const absoluteWindowsPathPattern = /^[a-zA-Z]:\\/;
const uncPathPattern = /^\\\\/;
let cachedAppDataDir: string | null = null;

async function getAppDataDirPath() {
  if (cachedAppDataDir) return cachedAppDataDir;
  cachedAppDataDir = await appDataDir();
  return cachedAppDataDir;
}

/**
 * 统一资源加载器
 * 策略：Local AppData -> Built-in Public Resource
 */
export async function loadWikiResource<T = unknown>(relativePath: string): Promise<T> {
  // 1. 如果是 Tauri 环境，优先尝试读取本地更新的数据
  if (IS_TAURI) {
    try {
      // 路径如: temp/info/index.json
      // 注意：BaseDirectory.AppData 对应的是 AppData/Roaming/com.tauri.dev (默认)
      // 我们在 Rust 中使用的是 app.path().app_data_dir()
      const existsInLocal = await exists(relativePath, {
        baseDir: BaseDirectory.AppData,
      });

      if (existsInLocal) {
        console.log(`[WikiLoader] Loading from LocalData: ${relativePath}`);
        const content = await readFile(relativePath, {
          baseDir: BaseDirectory.AppData,
        });
        const text = new TextDecoder().decode(content);
        return JSON.parse(text) as T;
      }
    } catch (e) {
      console.warn(`[WikiLoader] Failed to check local data for ${relativePath}`, e);
    }
  }

  // 2. 回退到内置资源 (public 目录)
  // 注意：在 Tauri 中，public 资源位于根路径
  // 如果 relativePath 是 "temp/info/index.json"，我们需要确保 publicUrl 是 "/temp/info/index.json"
  const publicUrl = relativePath.startsWith('/') ? relativePath : `/${relativePath}`;
  console.log(`[WikiLoader] Loading from Built-in: ${publicUrl}`);
  const response = await window.fetch(publicUrl);
  if (!response.ok) throw new Error(`Failed to load ${publicUrl}: ${response.statusText}`);
  return (await response.json()) as T;
}

export async function resolveAssetUrl(url: string): Promise<string> {
  if (!url) return url;
  if (!IS_TAURI || httpUrlPattern.test(url) || protocolUrlPattern.test(url)) return url;
  if (absoluteWindowsPathPattern.test(url) || uncPathPattern.test(url)) {
    return convertFileSrc(url);
  }
  const relativePath = url.startsWith('/') ? url.slice(1) : url;
  try {
    const base = await getAppDataDirPath();
    const fullPath = await join(base, relativePath);
    return convertFileSrc(fullPath);
  } catch {
    return url;
  }
}
