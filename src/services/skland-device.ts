import { invoke } from '@tauri-apps/api/core';

export function getDefaultUserAgent(): string {
  return typeof navigator !== 'undefined' ? navigator.userAgent : '';
}

function getCanvasFingerprint(): string {
  try {
    const canvas = document.createElement('canvas');
    const ctx = canvas.getContext('2d');
    if (!ctx) return '';
    const txt = 'http://www.ishumei.com';
    ctx.textBaseline = 'top';
    ctx.font = "24px 'Arial'";
    ctx.textBaseline = 'alphabetic';
    ctx.fillStyle = '#e88';
    ctx.fillRect(120, 1, 60, 22);
    ctx.fillStyle = '#f99';
    ctx.fillText(txt, 2, 15);
    ctx.fillStyle = 'rgba(120, 180, 0, 0.7)';
    ctx.fillText(txt, 4, 17);
    const base64 = canvas.toDataURL().replace('data:image/png;base64,', '');
    const raw = atob(base64).slice(-16, -12);
    let hex = '';
    for (let i = 0; i < raw.length; i += 1) {
      const code = raw.charCodeAt(i).toString(16);
      hex += code.length < 2 ? `0${code}` : code;
    }
    return hex;
  } catch {
    return '';
  }
}

function getPlugins(): string {
  try {
    const plugins = [];
    const pluginList = navigator.plugins ? Array.from(navigator.plugins) : [];
    for (const plugin of pluginList) {
      if (!plugin) continue;
      const description = plugin.description?.includes('Shockwave Flash')
        ? ''
        : (plugin.description ?? '');
      plugins.push(`${plugin.name ?? ''}${description}${plugin.filename ?? ''}${plugin.length ?? ''}`);
    }
    plugins.sort();
    const joined = plugins.join();
    return joined ? joined.replace(/\s/g, '') : '-';
  } catch {
    return '-';
  }
}

function createUid(): string {
  if (typeof crypto.randomUUID === 'function') {
    return crypto.randomUUID();
  }
  const bytes = crypto.getRandomValues(new Uint8Array(16));
  bytes[6] = ((bytes[6] ?? 0) & 0x0f) | 0x40;
  bytes[8] = ((bytes[8] ?? 0) & 0x3f) | 0x80;
  return Array.from(bytes, (b, i) => {
    const hex = b.toString(16).padStart(2, '0');
    return i === 4 || i === 6 || i === 8 || i === 10 ? `-${hex}` : hex;
  }).join('');
}

function generateFingerprint(did: string, userAgent: string) {
  const now = new Date();
  const timestamp = now
    .toISOString()
    .replace(/[-T:.Z]/g, '')
    .slice(0, 14);
  const randomHex = Array.from(crypto.getRandomValues(new Uint8Array(16)))
    .map((b) => b.toString(16).padStart(2, '0'))
    .join('');

  const smid = `${timestamp}${randomHex}`;
  const canvas = getCanvasFingerprint();
  const body = document.body;
  const windowWithMoz = window as Window & {
    mozInnerScreenX?: number;
    mozInnerScreenY?: number;
    screenLeft?: number;
    screenTop?: number;
  };
  const screenX = windowWithMoz.mozInnerScreenX ?? windowWithMoz.screenLeft ?? 0;
  const screenY = windowWithMoz.mozInnerScreenY ?? windowWithMoz.screenTop ?? 0;
  const clientSize = [
    screenX,
    screenY,
    body ? body.clientWidth : 0,
    body ? body.clientHeight : 0,
    screen.width,
    screen.height,
    screen.availWidth,
    screen.availHeight,
  ].join('_');
  const res = `${screen.width}_${screen.height}_${screen.colorDepth}_${window.devicePixelRatio}`;
  const url = window.location.href.slice(0, 64);
  const referer = document.referrer.slice(0, 64);
  const box = did ? did.replace(/^B/, '') : '';

  return {
    smid,
    canvas,
    clientSize,
    svm: now.getTime(),
    pmf: now.getTime(),
    ua: userAgent,
    plugins: getPlugins(),
    timezone: now.getTimezoneOffset(),
    platform: navigator.platform || '',
    url,
    referer,
    res,
    status: 0,
    vpw: createUid(),
    trees: createUid(),
    time: Math.round(performance.now()),
    box,
  };
}

export async function requestDid(currentDid: string, userAgent: string): Promise<string> {
  const fingerprint = generateFingerprint(currentDid, userAgent || getDefaultUserAgent());
  return await invoke<string>('generate_did', { fingerprint });
}
