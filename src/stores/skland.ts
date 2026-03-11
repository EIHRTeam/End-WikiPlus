import { defineStore } from 'pinia';
import { getDefaultUserAgent, requestDid } from 'src/services/skland-device';

const SK_PUBLIC_DID_KEY = 'SK_PUBLIC_DID_KEY';
const SK_PUBLIC_AUTO_FETCH_DID_KEY = 'SK_PUBLIC_AUTO_FETCH_DID_KEY';
const SK_PUBLIC_USER_AGENT_KEY = 'SK_PUBLIC_USER_AGENT_KEY';

function readBoolean(raw: string | null, fallback: boolean): boolean {
  if (raw === null) return fallback;
  return raw === 'true';
}

function persistPublicState(did: string, autoFetchDid: boolean, userAgent: string): void {
  if (did.trim()) {
    localStorage.setItem(SK_PUBLIC_DID_KEY, did.trim());
  } else {
    localStorage.removeItem(SK_PUBLIC_DID_KEY);
  }

  localStorage.setItem(SK_PUBLIC_AUTO_FETCH_DID_KEY, String(autoFetchDid));

  if (userAgent.trim()) {
    localStorage.setItem(SK_PUBLIC_USER_AGENT_KEY, userAgent.trim());
  } else {
    localStorage.removeItem(SK_PUBLIC_USER_AGENT_KEY);
  }
}

export const useSklandStore = defineStore('skland', {
  state: () => ({
    did: '',
    autoFetchDid: true,
    userAgent: '',
    initialized: false,
  }),
  getters: {
    hasDid: (state) => state.did.trim().length > 0,
  },
  actions: {
    loadPublicState() {
      this.did = (localStorage.getItem(SK_PUBLIC_DID_KEY) ?? '').trim();
      this.autoFetchDid = readBoolean(localStorage.getItem(SK_PUBLIC_AUTO_FETCH_DID_KEY), true);
      this.userAgent = (localStorage.getItem(SK_PUBLIC_USER_AGENT_KEY) ?? '').trim();

      if (!this.userAgent) {
        this.userAgent = getDefaultUserAgent();
      }
    },

    initialize() {
      if (this.initialized) return;
      this.loadPublicState();
      this.initialized = true;
    },

    saveDidSettings(newDid: string, autoFetchDid = true, userAgent = '') {
      this.did = newDid.trim();
      this.autoFetchDid = autoFetchDid;
      this.userAgent = (userAgent || this.userAgent || getDefaultUserAgent()).trim();
      persistPublicState(this.did, this.autoFetchDid, this.userAgent);
    },

    setAutoFetchDid(enabled: boolean) {
      this.autoFetchDid = enabled;
      persistPublicState(this.did, this.autoFetchDid, this.userAgent);
    },

    async ensureDid() {
      const existingDid = this.did.trim();
      if (existingDid) {
        return existingDid;
      }

      if (!this.autoFetchDid) {
        throw new Error('Automatic device ID generation is disabled');
      }

      const resolvedUserAgent = (this.userAgent || getDefaultUserAgent()).trim();
      const generatedDid = (await requestDid('', resolvedUserAgent)).trim();
      if (!generatedDid) {
        throw new Error('Generate device ID returned empty result');
      }

      this.saveDidSettings(generatedDid, this.autoFetchDid, resolvedUserAgent);
      return generatedDid;
    },

    async refreshDid() {
      const resolvedUserAgent = (this.userAgent || getDefaultUserAgent()).trim();
      const generatedDid = (await requestDid('', resolvedUserAgent)).trim();
      if (!generatedDid) {
        throw new Error('Generate device ID returned empty result');
      }

      this.saveDidSettings(generatedDid, this.autoFetchDid, resolvedUserAgent);
      return generatedDid;
    },
  },
});
