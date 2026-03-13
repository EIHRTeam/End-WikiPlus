declare namespace NodeJS {
  interface ProcessEnv {
    NODE_ENV: string;
    VUE_ROUTER_MODE: 'hash' | 'history' | 'abstract' | undefined;
    VUE_ROUTER_BASE: string | undefined;
  }
}

declare const __APP_VERSION__: string;
declare const __APP_BUILD_TIME__: string;

interface Window {
  __TAURI_INTERNALS__?: unknown;
}
