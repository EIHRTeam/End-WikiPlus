import type { RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      {
        path: '',
        name: 'wiki-list',
        component: () => import('pages/WikiListPage.vue'),
      },
      {
        path: 'wiki/list',
        name: 'wiki-list-legacy',
        redirect: (to) => ({ name: 'wiki-list', query: to.query }),
      },
      {
        path: 'wiki/item/:itemId',
        name: 'wiki-item',
        component: () => import('pages/WikiRendererPage.vue'),
      },
      {
        path: 'wiki/render',
        name: 'wiki-render',
        component: () => import('pages/WikiRendererPage.vue'),
      },
      {
        path: 'settings',
        name: 'settings',
        component: () => import('pages/SettingsPage.vue'),
      },
      {
        path: 'settings/permissions',
        name: 'permissions',
        component: () => import('pages/PermissionsPage.vue'),
      },
      {
        path: 'about',
        name: 'about',
        component: () => import('pages/AboutPage.vue'),
      },
      {
        path: 'readme',
        name: 'readme',
        component: () => import('pages/ReadmePage.vue'),
      },
      {
        path: 'user-agreement',
        name: 'user-agreement',
        component: () => import('pages/UserAgreementPage.vue'),
      },
      {
        path: 'privacy-policy',
        name: 'privacy-policy',
        component: () => import('pages/PrivacyPolicyPage.vue'),
      },
      {
        path: 'license',
        name: 'license',
        component: () => import('pages/LicensePage.vue'),
      },
      {
        path: 'third-party-licenses',
        name: 'third-party-licenses',
        component: () => import('pages/ThirdPartyLicensesPage.vue'),
      },
    ],
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/ErrorNotFound.vue'),
  },
];

export default routes;
