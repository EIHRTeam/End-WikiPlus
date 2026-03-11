import { defineRouter } from '#q-app/wrappers';
import {
  createMemoryHistory,
  createRouter,
  createWebHashHistory,
  createWebHistory,
  type LocationQueryRaw,
} from 'vue-router';
import routes from './routes';

/*
 * If not building with SSR mode, you can
 * directly export the Router instantiation;
 *
 * The function below can be async too; either use
 * async/await or return a Promise which resolves
 * with the Router instance.
 */

export default defineRouter(function (/* { store, ssrContext } */) {
  const createHistory = process.env.SERVER
    ? createMemoryHistory
    : process.env.VUE_ROUTER_MODE === 'history'
      ? createWebHistory
      : createWebHashHistory;

  const Router = createRouter({
    scrollBehavior: (_to, _from, savedPosition) => {
      if (savedPosition) return savedPosition;
      return { left: 0, top: 0 };
    },
    routes,

    // Leave this as is and make changes in quasar.conf.js instead!
    // quasar.conf.js -> build -> vueRouterMode
    // quasar.conf.js -> build -> publicPath
    history: createHistory(process.env.VUE_ROUTER_BASE),
  });

  Router.beforeEach((to) => {
    if (to.path !== '/' && to.path !== '/wiki/list') {
      return true;
    }

    const legacyItemId = to.query.itemId;
    const itemId = typeof legacyItemId === 'string' ? legacyItemId.trim() : '';
    if (!itemId) {
      return true;
    }

    const query: LocationQueryRaw = { ...to.query };
    delete query.itemId;

    if (import.meta.env.DEV) {
      console.info('[router] migrating legacy itemId query route', {
        from: to.fullPath,
        itemId,
      });
    }

    return {
      name: 'wiki-item',
      params: { itemId },
      query,
      replace: true,
    };
  });

  return Router;
});
