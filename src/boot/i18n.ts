import { defineBoot } from '#q-app/wrappers';
import { createI18n } from 'vue-i18n';

import messages from 'src/i18n';

export type MessageLanguages = keyof typeof messages;

type ToGeneric<T> = T extends string
  ? string
  : T extends object
    ? { [K in keyof T]: ToGeneric<T[K]> }
    : T;

// Type-define 'en-US' as master schema for resource keys (not values)
export type MessageSchema = ToGeneric<(typeof messages)['en-US']>;

// See https://vue-i18n.intlify.dev/guide/advanced/typescript.html#global-resource-schema-type-definition
/* eslint-disable @typescript-eslint/no-empty-object-type */
declare module 'vue-i18n' {
  // define the locale messages schema
  export interface DefineLocaleMessage extends MessageSchema {}

  // define the datetime format schema
  export interface DefineDateTimeFormat {}

  // define the number format schema
  export interface DefineNumberFormat {}
}
/* eslint-enable @typescript-eslint/no-empty-object-type */

export default defineBoot(({ app }) => {
  // 探测浏览器语言
  const browserLang = navigator.language;
  const detectedLocale: MessageLanguages =
    browserLang === 'zh-TW' || browserLang === 'zh-HK' || browserLang === 'zh-MO'
      ? 'zh-TW'
      : browserLang.startsWith('zh')
        ? 'zh-CN'
        : browserLang.startsWith('ja')
          ? 'ja-JP'
          : 'en-US';

  // 从 localStorage 读取语言设置，默认为浏览器探测的语言
  let initialLocale: MessageLanguages = detectedLocale;
  try {
    const raw = localStorage.getItem('jei.settings');
    if (raw) {
      const parsed = JSON.parse(raw);
      if (
        parsed.language === 'zh-CN' ||
        parsed.language === 'zh-TW' ||
        parsed.language === 'en-US' ||
        parsed.language === 'ja-JP'
      ) {
        initialLocale = parsed.language;
      }
    }
  } catch {
    // 忽略错误，使用默认值
  }

  const i18n = createI18n<{ message: MessageSchema }, MessageLanguages>({
    locale: initialLocale,
    legacy: false,
    messages,
  });

  // Set i18n instance on app
  app.use(i18n);
});
